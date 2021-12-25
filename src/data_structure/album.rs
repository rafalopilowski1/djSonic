use serde::Deserialize;

use crate::data_structure::child::Child;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AlbumID3 {
    id: String,
    name: String,
    artist: Option<String>,
    artist_id: Option<String>,
    cover_art: Option<String>,
    song_count: u32,
    duration: u32,
    play_count: Option<u64>,
    created: String,
    starred: Option<String>,
    year: Option<u32>,
    genre: Option<String>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AlbumWithSongsID3 {
    album: AlbumID3,
    songs: Vec<Child>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AlbumInfo {
    notes: Option<String>,
    music_brainz_id: Option<String>,
    last_fm_url: Option<String>,
    small_image_url: Option<String>,
    medium_image_url: Option<String>,
    large_image_url: Option<String>,
}
