use std::time::Duration;

use serde::Deserialize;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Child {
    id: String,
    parent: Option<String>,
    is_dir: bool,
    title: String,
    album: Option<String>,
    track: Option<u32>,
    year: Option<u32>,
    genre: Option<String>,
    cover_art: Option<String>,
    size: Option<u64>,
    content_type: Option<String>,
    suffix: Option<String>,
    transcoded_content_type: Option<String>,
    transcoded_suffix: Option<String>,
    duration: Option<u32>,
    bit_rate: Option<u32>,
    path: Option<String>,
    is_video: Option<bool>,
    user_rating: Option<u32>,    // UserRating: [1,5]
    average_rating: Option<f32>, // AverageRating: [1.0,5.0]
    play_count: Option<u64>,
    disc_number: Option<u32>,
    created: Option<Duration>,
    starred: Option<Duration>,
    album_id: Option<String>,
    artist_id: Option<String>,
    media_type: Option<MediaType>,
    bookmark_position: Option<u64>,
    original_width: Option<u32>,
    original_height: Option<u32>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum MediaType {
    Music,
    Podcast,
    Audiobook,
    Video,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NowPlaying {
    #[serde(rename="$value")]
    entries: Vec<NowPlayingEntry>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NowPlayingEntry {
    child: Child,
    username: String,
    minutes_ago: u32,
    player_id: u32,
    player_name: Option<String>,
}
