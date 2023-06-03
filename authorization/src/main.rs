use std::env;

use askama_axum::IntoResponse;
use axum::routing::{get, post};
use axum::{extract::Form, Router};
use res_req::{AccessTokenRequest, DeviceAuthorizationRequest, LoginForm};
use tracing::instrument;

use crate::res_req::{AccessTokenResponse, DeviceAuthorizationResponse, LoginResponse};

mod res_req;

#[tokio::main]
async fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".into());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt()
        // .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        // .compact()
        .init();
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/device_authorization", post(accept_form))
        .route("/device", get(login).post(submit))
        .route("/token", post(token));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[instrument(ret)]
async fn accept_form(
    Form(input): Form<DeviceAuthorizationRequest>,
) -> axum::Json<DeviceAuthorizationResponse> {
    let uri = "http://localhost/device";
    let user_code = "WDJB-MJHT";
    let body = DeviceAuthorizationResponse {
        device_code: "GmRhmhcxhwAzkoEqiMEg_DnyEysNkuNhszIySk9eS".to_owned(),
        user_code: user_code.to_owned(),
        verification_uri: uri.to_owned(),
        verification_uri_complete: format!("{uri}/{user_code}"),
        expires_in: 1800,
        interval: 5,
    };
    axum::Json(body)
}
#[instrument(ret)]
async fn login() -> impl IntoResponse {
    LoginForm::default()
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
