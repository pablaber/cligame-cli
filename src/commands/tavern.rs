use clap::Command;
use crate::api::client::ApiClient;
pub fn command() -> Command {
    Command::new("tavern").about("Visit the tavern")
}

pub async fn run(api_client: &ApiClient) {
    if !api_client.has_creds {
        println!("You are not logged in. Please login first.");
        return;
    }

    let tavern_response = api_client.tavern().await;
    let tavern_response = match tavern_response {
        Ok(tavern_response) => tavern_response,
        Err(e) => {
            e.print();
            return;
        }
    };

    println!("Welcome to the tavern, {}!", tavern_response.user.character.name);
}
