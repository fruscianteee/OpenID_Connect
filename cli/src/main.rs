use anyhow::Result;
use authorization::res_req::AccessTokenRequest;
use authorization::res_req::AccessTokenResponse;
use authorization::res_req::DeviceAuthorizationRequest;
use authorization::res_req::DeviceAuthorizationResponse;
use authorization::res_req::ErrorResponse;
use authorization::res_req::ErrorResponseKind;

#[tokio::main]
async fn main() -> Result<()> {
    let authorization_server = "http://localhost:3000";

    let body = reqwest::get(authorization_server).await?.text().await?;
    println!("body = {:?}", body);

    // client_id=1406020730&scope=example_scope
    let client_id = 0;
    let client = reqwest::Client::new();
    let device_authorization_request = dbg!(DeviceAuthorizationRequest {
        client_id: client_id.to_string(),
        scope: "suco".to_owned()
    });

    let device_authorization_response = client
        .post(format!("{authorization_server}/device_authorization"))
        .form(&device_authorization_request)
        .send()
        .await?
        .json::<DeviceAuthorizationResponse>()
        .await?;
    dbg!(&device_authorization_response);

    // polling fetch token
    let access_token_response = loop {
        let access_token_request = dbg!(AccessTokenRequest::DeviceCode {
            device_code: device_authorization_response.device_code.clone(),
            client_id: client_id.to_string()
        });
        let result = client
            .post(format!("{authorization_server}/token"))
            .form(&access_token_request)
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
                            anyhow::bail!("error");
                        }
                    }
                }
                response.json::<AccessTokenResponse>().await?
            }
            Err(error) => {
                dbg!(error);
                anyhow::bail!("error");
            }
        };
    };
    dbg!(access_token_response);
    Ok(())
}
