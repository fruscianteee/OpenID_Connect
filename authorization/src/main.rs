use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::sync::RwLock;

use askama_axum::IntoResponse;
use axum::extract::Query;
use axum::routing::{get, post};
use axum::{extract::Form, Router};
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use res_req::{AccessTokenRequest, DeviceAuthorizationRequest, LoginForm};
use serde::Deserialize;
use tracing::instrument;

use crate::res_req::{AccessTokenResponse, DeviceAuthorizationResponse, LoginResponse};

mod res_req;

type DeviceCode = [u8; 32];
type UserCode = String;
struct DeviceAuthorization {
    pub device_code: DeviceCode,
    pub user_code: UserCode,
}

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".into());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt()
        // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        // .compact()
        .init();
    let device_authorizations = Arc::new(RwLock::new(HashMap::new()));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/device_authorization", {
            post({
                let device_authorizations = device_authorizations.clone();
                move |Form(req)| async { device_authorization(req, device_authorizations) }
            })
        })
        .route("/device", get(login).post(submit))
        .route("/token", post(token));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[instrument(skip(device_authorizations), ret)]
fn device_authorization(
    input: DeviceAuthorizationRequest,
    device_authorizations: Arc<RwLock<HashMap<String, DeviceAuthorization>>>,
) -> axum::Json<DeviceAuthorizationResponse> {
    let device_code: DeviceCode = {
        let mut rng = rand::thread_rng();
        rng.gen()
    };
    let user_code: UserCode = unsafe {
        String::from_utf8_unchecked(thread_rng().sample_iter(&Alphanumeric).take(8).collect())
    };
    {
        let mut device_authorizations = device_authorizations.write().unwrap();
        device_authorizations.insert(
            input.client_id.clone(),
            DeviceAuthorization {
                device_code: device_code.clone(),
                user_code: user_code.clone(),
            },
        );
    }
    let uri = "http://localhost:3000/device";
    let user_code = format!(
        "{}-{}",
        user_code.get(..4).unwrap(),
        user_code.get(4..).unwrap()
    );
    let body = DeviceAuthorizationResponse {
        device_code: hex::encode(device_code),
        user_code: user_code.to_owned(),
        verification_uri: uri.to_owned(),
        verification_uri_complete: format!("{uri}?user_code={user_code}"),
        expires_in: 0,
        interval: 5,
    };
    axum::Json(body)
}

#[derive(Deserialize)]
struct LoginQuery {
    user_code: String,
}

#[instrument(skip(query), ret)]
async fn login(Query(query): Query<LoginQuery>) -> impl IntoResponse {
    LoginForm {
        user_code: query.user_code,
        ..Default::default()
    }
}

#[instrument(ret)]
async fn submit(Form(input): Form<LoginForm>) -> impl IntoResponse {
    dbg!(&input);
    // instantiate your struct
    LoginResponse {
        user: input,
        result_msg: "認証OK !!!!!".to_owned(),
    }
}

#[instrument(ret)]
async fn token(Form(input): Form<AccessTokenRequest>) -> impl IntoResponse {
    dbg!(&input);
    // instantiate your struct
    axum::Json(AccessTokenResponse::default())
}
