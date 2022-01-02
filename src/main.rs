use data_structure::album::AlbumListType;

use crate::api::subsonic_client::SubsonicClient;
use std::error::Error;

mod api;
mod data_structure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let API_ENDPOINT = dotenv::var("API_ENDPOINT")?;
    let user = dotenv::var("SUBSONIC_USER")?;
    let password = dotenv::var("SUBSONIC_PASSWORD")?;

    let mut subsonic_client = SubsonicClient::new(&API_ENDPOINT, &user, &password);
    SubsonicClient::init(&mut subsonic_client).await;

    if let Some(artists) = subsonic_client.get_artists().await? {
        println!("{:#?}", artists);
    };
    if let Some(genres) = subsonic_client.get_genres().await? {
        println!("{:#?}", genres);
    };
    // if let Some(directory) = subsonic_client.get_directory().await? {
    //         println!("{:#?}",directory);
    // };
    if let Some(music_folders) = subsonic_client.get_music_folders().await? {
        println!("{:#?}", music_folders);
    };
    if let Some(indexes) = subsonic_client.get_indexes().await? {
        println!("{:#?}", indexes);
    };

    // if let Some(bookmarks) = subsonic_client.get_bookmarks().await? {
    //         println!("{:#?}", bookmarks);
    // };

    // SLOW!!!

    // if let Some(podcasts) = subsonic_client.get_podcasts().await? {
    //     println!("{:#?}", podcasts);
    // };

    // if let Some(nowPlaying) = subsonic_client.get_now_playing().await? {
    //     println!("{:#?}", nowPlaying);
    // };

    if let Some(playlists) = subsonic_client.get_playlists().await? {
        println!("{:#?}", playlists);
    }
    if let Some(randomSongs) = subsonic_client.get_random_songs().await? {
        println!("{:#?}", randomSongs);
    }
    if let Some(newestPodcasts) = subsonic_client.get_newest_podcasts().await? {
        println!("{:#?}", newestPodcasts);
    }
    if let Some(albumList2) = subsonic_client
        .get_album_list_2(AlbumListType::Newest)
        .await?
    {
        println!("{:#?}", albumList2);
    }
    if let Some(value) = subsonic_client.ping().await? {
        println!("{:#?}", value);
    }
    if let Some(search) = subsonic_client.search3("florence").await? {
        println!("{:#?}", search);
    }
    Ok(())
}
