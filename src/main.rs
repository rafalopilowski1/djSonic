use tokio::{fs::File, io::AsyncWriteExt};

use crate::api::subsonic_client::SubsonicClient;
use std::error::Error;

mod api;
mod data_structure;

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

    // let searchResult = subsonic_client.search3("piątek").await;
    // println!("{:#?}", searchResult);

    // if let Ok(Some(search)) = searchResult {
    //     for element in search.getValues().unwrap() {
    //         if let Some((cover_art_bytes, cover_art_id)) =
    //             subsonic_client.get_cover_art(element).await?
    //         {
    //             // cover art - JPGs as binary
    //             let mut file = File::create(format!("{}.jpg", cover_art_id)).await?;
    //             file.write_all(&cover_art_bytes).await?;
    //             file.sync_all().await?;
    //         }
    //     }
    // };

    // if let Some(song) = subsonic_client.get_song(1).await? {
    //     println!("{:#?}", song);
    // }
    // if let Some(song) = subsonic_client.get_album(1).await? {
    //     println!("{:#?}", song);
    // }
    if let Some(artist) = subsonic_client.get_artist(1).await? {
        println!("{:#?}", artist);
    }
    Ok(())
}
