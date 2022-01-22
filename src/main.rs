use discord::commands;
use reqwest::header::CONTENT_TYPE;
use serenity::framework::StandardFramework;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::api::subsonic_client::SubsonicClient;
use songbird::SerenityInit;
use std::error::Error;

mod api;
mod data_structure;
mod discord;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let API_ENDPOINT = dotenv::var("API_ENDPOINT")?;
    let user = dotenv::var("SUBSONIC_USER")?;
    let password = dotenv::var("SUBSONIC_PASSWORD")?;

    let subsonic_client = SubsonicClient::new(&API_ENDPOINT, &user, &password).await?;

    let token = dotenv::var("DISCORD_TOKEN")?;
    let application_id: u64 = dotenv::var("APPLICATION_ID")?.parse()?;
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
