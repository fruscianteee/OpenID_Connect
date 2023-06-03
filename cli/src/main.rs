use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
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

#[derive(Debug, Serialize)]
#[serde(tag = "grant_code")]
enum AccessTokenRequest {
    #[serde(rename = "urn:ietf:params:oauth:grant-type:device_code")]
    DeviceCode {
        device_code: String,
        client_id: String,
    },
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<u32>,
    refresh_token: Option<String>,
    example_parameter: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: ErrorResponseKind,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ErrorResponseKind {
    AuthorizationPending,
    SlowDown,
    Others,
}

const DEVICE_FLOW_GRANT_TYPE: &'static str = "urn:ietf:params:oauth:grant-type:device_code";

#[tokio::main]
async fn main() -> Result<()> {
    let authorization_server = "http://localhost:3000";

    let body = reqwest::get(authorization_server).await?.text().await?;
    println!("body = {:?}", body);

    // client_id=1406020730&scope=example_scope
    let client_id = 0.to_string();
    let client = reqwest::Client::new();
    let mut params = HashMap::new();
    params.insert("client_id", client_id.to_string());
    params.insert("scope", "suco".to_string());

    let device_authorization_response = client
        .post(format!("{authorization_server}/device_authorization"))
        .form(&params)
        .send()
        .await?
        .json::<DeviceAuthorizationResponse>()
        .await?;
    dbg!(&device_authorization_response);

    // polling fetch token
    let access_token_response = loop {
        let request = dbg!(AccessTokenRequest::DeviceCode {
            device_code: device_authorization_response.device_code.clone(),
            client_id: client_id.to_owned()
        });
        let result = client
            .post(format!("{authorization_server}/token"))
            .form(&request)
            .send()
            .await;
        break match result {
            Ok(response) => {
                if !response.status().is_success() {
                    let error_response = response.json::<ErrorResponse>().await?;
                    dbg!(&error_response);
                    match error_response.error {
                        ErrorResponseKind::AuthorizationPending => {
                            dbg!("AuthorizationPending");
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            continue;
                        }
                        ErrorResponseKind::SlowDown => {
                            dbg!("SlowDown");
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                            continue;
                        }
                        ErrorResponseKind::Others => {
                            return anyhow::bail!("error");
                        }
                    }
                }
                response.json::<AccessTokenResponse>().await?
            }
            Err(error) => {
                dbg!(error);
                return anyhow::bail!("error");
            }
        };
    };
    dbg!(access_token_response);
    Ok(())
}
