use axum::routing::{get, post};
use axum::Json;
use axum::{extract::Form, response::Html, Router};
use serde::{Deserialize, Serialize};

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
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/device_authorization", post(accept_form));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

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
