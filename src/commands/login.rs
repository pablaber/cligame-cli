use clap::{Arg, ArgMatches, Command};

use crate::api::client::ApiClient;

pub fn command() -> Command {
    Command::new("login")
        .about("Login to the user account.")
        .arg(
            Arg::new("email")
                .long("email")
                .short('e')
                .help("Email address")
                .required(true),
        )
        .arg(
            Arg::new("password")
                .long("password")
                .short('p')
                .help("Password")
                .required(true),
        )
}

pub async fn run(matches: &ArgMatches, api_client: &mut ApiClient) {
    let email = matches.get_one::<String>("email").unwrap();
    let password = matches.get_one::<String>("password").unwrap();

    let login_response = api_client.login(email, password).await;
    let login_response = match login_response {
        Ok(login_response) => login_response,
        Err(e) => {
            e.print();
            return;
        }
    };

    api_client.set_access_token(login_response.get_access_token());
    api_client.set_refresh_token(login_response.get_refresh_token());
    let save_result = api_client.save_creds_to_keyring();
    if save_result.is_err() {
        println!("Failed to save credentials to file: {}", save_result.err().unwrap());
    } else {
        println!("Logged in as {}", login_response.user.email);
    }
}
