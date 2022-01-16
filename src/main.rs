use discord::commands;
use reqwest::header::CONTENT_TYPE;
use serenity::framework::StandardFramework;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::api::subsonic_client::SubsonicClient;
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

    // let artists = subsonic_client.get_artists().await;
    // println!("{:#?}", artists);
    // let genres = subsonic_client.get_genres().await;
    // println!("{:#?}", genres);
    // let directory = subsonic_client.get_directory().await;
    // println!("{:#?}", directory);
    // let music_folders = subsonic_client.get_music_folders().await;
    // println!("{:#?}", music_folders);
    // let indexes = subsonic_client.get_indexes().await;
    // println!("{:#?}", indexes);
    // let bookmarks = subsonic_client.get_bookmarks().await;
    // println!("{:#?}", bookmarks);
    // let podcasts = subsonic_client.get_podcasts().await;
    // println!("{:#?}", podcasts);
    // let nowPlaying = subsonic_client.get_now_playing().await;
    // println!("{:#?}", nowPlaying);
    // let playlists = subsonic_client.get_playlists().await;
    // println!("{:#?}", playlists);
    // if let Some(playlist) = subsonic_client.get_playlist(1).await? {
    //     println!("{:#?}", playlist);
    // };
    // let randomSongs = subsonic_client.get_random_songs().await;
    // println!("{:#?}", randomSongs);
    // let newestPodcasts = subsonic_client.get_newest_podcasts().await;
    // println!("{:#?}", newestPodcasts);
    // let albumList2 = subsonic_client
    //     .get_album_list_2(AlbumListType::Newest)
    //     .await;
    // println!("{:#?}", albumList2);
    // let pingInfo = subsonic_client.ping().await;
    // println!("{:#?}", pingInfo);
    // let user = subsonic_client.getUser().await;
    // println!("{:#?}", user);

    // let searchResult = subsonic_client.search3("25").await;
    // println!("{:#?}", searchResult);

    // if let Ok(Some(search)) = searchResult {
    //     for element in search.getValues().unwrap() {
    //         if let Some((cover_art_bytes, file_path)) =
    //             subsonic_client.get_cover_art(element).await?
    //         {
    //             let mut file = File::create(format!("{}.jpg", file_path)).await?;
    //             file.write_all(&cover_art_bytes).await?;
    //             file.sync_all().await?;
    //         }
    //     }
    // };

    // if let Some(song) = subsonic_client.get_song(1).await? {
    //     println!("{:#?}", song);
    //     if let Some((song_stream, file_name)) = subsonic_client.stream(song).await? {
    //         let mut file = File::create(file_name).await?;
    //         file.write_all(&song_stream).await?;
    //         file.sync_all().await?
    //     }
    // }
    // if let Some(song) = subsonic_client.get_album(1).await? {
    //     println!("{:#?}", song);
    // }
    // if let Some(artist) = subsonic_client.get_artist(1).await? {
    //     println!("{:#?}", artist);
    //     if let Some(artistInfo2) = subsonic_client.get_artist_info_2(artist).await? {
    //         println!("{:#?}", artistInfo2);
    //     }
    // }

    let token = dotenv::var("DISCORD_TOKEN")?;
    let application_id: u64 = dotenv::var("APPLICATION_ID")?.parse()?;
    let framework = StandardFramework::new();
    let mut discord_client = serenity::Client::builder(token)
        .event_handler(commands::Handler::new(subsonic_client))
        .application_id(application_id)
        .framework(framework)
        .await?;
    if let Err(err) = discord_client.start().await {
        println!("{}", err);
    }
    Ok(())
}
