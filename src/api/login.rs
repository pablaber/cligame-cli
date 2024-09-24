use serde::{Serialize,Deserialize};
use super::{client::ApiClient, client::ApiError, types::ApiUser};


#[derive(Serialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub user: ApiUser,
    pub tokens: LoginResponseTokens,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponseTokens {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
}

impl LoginResponse {
    pub fn get_access_token(&self) -> &str {
        &self.tokens.access_token
    }

    pub fn get_refresh_token(&self) -> &str {
        &self.tokens.refresh_token
    }
}

impl ApiClient {
    pub async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, ApiError> {
        let login_request = LoginRequest {
            email: email.to_string(), password: password.to_string(),
        };

        let response = self.client.post(self.url("/auth/login")).json(&login_request).send().await;
        let response = match response {
            Ok(response) => response,
            Err(e) => {
                return Err(ApiError::new("ClientRequestError", e.to_string().as_str()))
            }
        };

        return self.unwrap_api_response::<LoginResponse>(response).await;
    }
}