use crate::data_structure::child::Child;
use std::time::Duration;

pub(crate) struct AlbumID3 {
    id: String,
    name: String,
    artist: Option<String>,
    artist_id: Option<String>,
    cover_art: Option<String>,
    song_count: u32,
    duration: u32,
    play_count: Option<u64>,
    created: Duration,
    starred: Option<Duration>,
    year: Option<u32>,
    genre: Option<String>,
}
pub(crate) struct AlbumWithSongsID3 {
    album: AlbumID3,
    songs: Vec<Child>,
}

pub(crate) struct AlbumInfo {
    notes: Option<String>,
    music_brainz_id: Option<String>,
    last_fm_url: Option<String>,
    small_image_url: Option<String>,
    medium_image_url: Option<String>,
    large_image_url: Option<String>,
}
