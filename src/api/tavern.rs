use super::{client::ApiClient, client::ApiError, types::ApiUser};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TavernResponse {
    pub user: ApiUser,
}

impl ApiClient {
    pub async fn tavern(&self) -> Result<TavernResponse, ApiError> {
        let response = self
            .client
            .get(self.url("/tavern"))
            .header(
                "Authorization",
                format!("Bearer {}", self.access_token.as_ref().unwrap()),
            )
            .send()
            .await;
        let response = match response {
            Ok(response) => response,
            Err(e) => return Err(ApiError::new("ClientRequestError", e.to_string().as_str())),
        };

        return self.unwrap_api_response::<TavernResponse>(response).await;
    }
}
