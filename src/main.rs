use std::error::Error;

mod api;
mod data_structure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let API_ENDPOINT = dotenv::var("API_ENDPOINT")?;
    let user = dotenv::var("SUBSONIC_USER")?;
    let password = dotenv::var("SUBSONIC_PASSWORD")?;
    let subsonic_client =
        crate::api::subsonic_client::SubsonicClient::new(&API_ENDPOINT, &user, &password);
    let artists = subsonic_client.get_artists().await?;
    println!("{:#?}", artists);
    Ok(())
}
