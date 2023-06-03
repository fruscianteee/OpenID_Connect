use std::env;

use askama::Template;
use askama_axum::IntoResponse;
use axum::http::Response;
use axum::routing::{get, post};
use axum::Json;
use axum::{extract::Form, response::Html, Router};
use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Serialize, Default)]
struct DeviceAuthorizationResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    verification_uri_complete: String,
    expires_in: u32,
    interval: u32,
}

#[derive(Debug, Deserialize, Default)]
struct DeviceAuthorizationRequest {
    client_id: u64,
    scope: String,
}

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
        .route("/device", get(login).post(submit));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[instrument(ret)]
async fn accept_form(
    Form(input): Form<DeviceAuthorizationRequest>,
) -> Json<DeviceAuthorizationResponse> {
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

#[derive(Template, Debug, Default, Deserialize, Serialize)]
#[template(path = "index.html")]
struct LoginForm {
    user_id: String,
    password: String,
    user_code: String,
}
#[instrument(ret)]
async fn login() -> impl IntoResponse {
    LoginForm::default()
}

#[derive(Template, Debug, Serialize)]
#[template(path = "response.html")] // もしあなたが結果を表示するための別のテンプレートを持っているなら、そのパスを指定してください。
struct LoginResponse {
    result_msg: String,
    user: LoginForm,
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
