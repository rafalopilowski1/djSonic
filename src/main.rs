use discord::commands;

use serenity::framework::StandardFramework;

use crate::api::subsonic_client::SubsonicClient;
use songbird::SerenityInit;
use std::error::Error;

mod api;
mod data_structure;
mod discord;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let API_ENDPOINT = std::env::var("API_ENDPOINT")?;
    let user = std::env::var("SUBSONIC_USER")?;
    let password = std::env::var("SUBSONIC_PASSWORD")?;

    let subsonic_client = SubsonicClient::new(&API_ENDPOINT, &user, &password)
        .await
        .expect("SubSonic server not responsing! Check your internet connection or server status.");

    let token = std::env::var("DISCORD_TOKEN")?;
    let application_id: u64 = std::env::var("APPLICATION_ID")?.parse()?;
    let framework = StandardFramework::new();
    let mut discord_client = serenity::Client::builder(token)
        .event_handler(commands::Handler::new(subsonic_client))
        .application_id(application_id)
        .framework(framework)
        .register_songbird()
        .await?;
    if let Err(err) = discord_client.start().await {
        println!("{}", err);
    }
    Ok(())
}
