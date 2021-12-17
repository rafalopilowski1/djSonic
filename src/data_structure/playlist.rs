use std::time::Duration;

use crate::data_structure::child::Child;

pub(crate) struct Playlist {
    allowed_users: Vec<String>,
    id: String,
    name: String,
    comment: Option<String>,
    owner: Option<String>,
    public: Option<bool>,
    song_count: u32,
    duration: u32,
    created: Duration,
    changed: Duration,
    cover_art: Option<String>,
}
pub(crate) struct PlaylistWithSongs {
    playlist: Playlist,
    entries: Vec<Child>,
}
