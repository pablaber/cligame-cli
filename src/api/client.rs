use reqwest::{Client, ClientBuilder, Response};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use keyring::Entry;
use std::{fs::File, io::BufReader};
pub struct ApiClient {
    pub client: Client,
    pub has_creds: bool,
    entry: Entry,
    api_base_url: String,
    pub access_token: Option<String>,
    refresh_token: Option<String>,
}

#[derive(Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError,
}

#[derive(Deserialize)]
pub struct ApiError {
    name: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    access_token: String,
    refresh_token: String,
}

impl ApiError {
    pub fn new(name: &str, message: &str) -> ApiError {
        ApiError {
            name: name.to_string(), message: message.to_string()
        }
    }

    pub fn print(&self) {
        println!("{}: {}", self.name, self.message);
    }
}

impl ApiClient {
    pub fn new(api_base_url: &str) -> ApiClient {
        let mut api_client = ApiClient {
            client: ClientBuilder::new().build().unwrap(),
            api_base_url: api_base_url.to_string(),
            access_token: None,
            refresh_token: None,
            entry: Entry::new("cligame", "api_credentials").unwrap(),
            has_creds: false,
        };

        let creds = api_client.load_creds_from_keyring();
        match creds {
            Ok(_) => {
                api_client.has_creds = true;
            },
            Err(_) => {
                api_client.has_creds = false;
            },
        }

        api_client
    }

    pub fn new_with_creds_file(api_base_url: &str, creds_file: &str) -> ApiClient {
        // Read creds file
        let mut api_client = ApiClient {
            client: ClientBuilder::new().build().unwrap(),
            api_base_url: api_base_url.to_string(),
            access_token: None,
            refresh_token: None,
            entry: Entry::new("cligame", "api_credentials").unwrap(),
            has_creds: false,
        };

        let creds = api_client.load_creds_from_config_file(creds_file);
        match creds {
            Ok(_) => {
                api_client.has_creds = true;
            },
            Err(_) => {
                api_client.has_creds = false;
            },
        }

        api_client
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.api_base_url, path)
    }

    pub async fn unwrap_api_response<T: DeserializeOwned>(&self, response: Response) -> Result<T, ApiError> {
        if !response.status().is_success() {
            let json = response.json::<ApiErrorResponse>().await;
            match json {
                Ok(json) => return Err(json.error),
                Err(e) => return Err(ApiError::new("UnexpectedJsonError", e.to_string().as_str())),
            }
        }

        let json = response.json::<T>().await;
        match json {
            Ok(json) => Ok(json),
            Err(e) => Err(ApiError::new("UnexpectedJsonError", e.to_string().as_str())),
        }
    }

    pub fn set_access_token(&mut self, access_token: &str) {
        self.access_token = Some(access_token.to_string());
    }

    pub fn set_refresh_token(&mut self, refresh_token: &str) {
        self.refresh_token = Some(refresh_token.to_string());
    }

    pub fn save_creds_to_keyring(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.access_token.is_none() || self.refresh_token.is_none() {
            eprintln!("No credentials to save");
            return Ok(());
        }

        let creds = Credentials {
            access_token: self.access_token.clone().unwrap(),
            refresh_token: self.refresh_token.clone().unwrap(),
        };

        let string_credentials = serde_json::to_string(&creds)?;
        self.entry.set_password(&string_credentials)?;

        Ok(())
    }

    pub fn load_creds_from_keyring(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let credentials_string = self.entry.get_password()?;
        let creds: Credentials = serde_json::from_str(&credentials_string)?;

        self.set_access_token(&creds.access_token);
        self.set_refresh_token(&creds.refresh_token);

        Ok(())
    }

    pub fn load_creds_from_config_file(&mut self, creds_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(creds_file)?;
        let reader = BufReader::new(file);

        let creds: Credentials = serde_json::from_reader(reader)?;

        self.set_access_token(&creds.access_token);
        self.set_refresh_token(&creds.refresh_token);

        Ok(())
    }
}
