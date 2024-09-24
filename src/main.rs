use clap::{command, Arg};

mod api;
mod commands;

use api::client::ApiClient;

#[tokio::main]
async fn main() {
    let matches = command!()
        .arg(
            Arg::new("creds-file")
                .long("creds-file")
                .short('c')
                .help("Config file"),
        )
        .subcommand_required(true)
        .subcommand(commands::login::command())
        .subcommand(commands::tavern::command())
        .get_matches();

    let mut api: ApiClient;

    let creds_file = matches.get_one::<String>("creds-file");
    
    match creds_file {
        Some(creds_file) => {
            api = ApiClient::new_with_creds_file("http://localhost:3000", creds_file);
        }
        None => {
            api = ApiClient::new("http://localhost:3000");
        }
    }

    match matches.subcommand() {
        Some(("login", sub_matches)) => commands::login::run(sub_matches, &mut api).await,
        Some(("tavern", _matches)) => commands::tavern::run(&api).await,

        _ => unreachable!(),
    }
}
