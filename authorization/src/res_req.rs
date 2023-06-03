use askama::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceAuthorizationResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: u32,
    pub interval: u32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeviceAuthorizationRequest {
    pub client_id: String,
    pub scope: String,
}

#[derive(Template, Debug, Default, Deserialize, Serialize)]
#[template(path = "index.html")]
pub struct LoginForm {
    pub user_id: String,
    pub password: String,
    pub user_code: String,
}

#[derive(Template, Debug, Serialize, Deserialize)]
#[template(path = "response.html")] // もしあなたが結果を表示するための別のテンプレートを持っているなら、そのパスを指定してください。
pub struct LoginResponse {
    pub result_msg: String,
    pub user: LoginForm,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "grant_code")]
pub enum AccessTokenRequest {
    #[serde(rename = "urn:ietf:params:oauth:grant-type:device_code")]
    DeviceCode {
        device_code: String,
        client_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u32>,
    pub refresh_token: Option<String>,
    pub example_parameter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorResponseKind,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorResponseKind {
    AuthorizationPending,
    SlowDown,
    Others,
}
