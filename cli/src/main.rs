use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct DeviceAuthorizationResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    verification_uri_complete: String,
    expires_in: u32,
    interval: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let authorization_server = "http://localhost:3000";

    let body = reqwest::get(authorization_server).await?.text().await?;
    println!("body = {:?}", body);

    // client_id=1406020730&scope=example_scope
    let client = reqwest::Client::new();
    let mut params = HashMap::new();
    params.insert("client_id", "0");
    params.insert("scope", "suco");

    let res = client
        .post(format!("{authorization_server}/device_authorization"))
        .form(&params)
        .send()
        .await?
        .json::<DeviceAuthorizationResponse>()
        .await?;
    dbg!(res);
    Ok(())
}
