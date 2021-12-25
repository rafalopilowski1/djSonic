

use serde::Deserialize;

use crate::data_structure::child::Child;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Playlist {
    allowed_users: Vec<String>,
    id: String,
    name: String,
    comment: Option<String>,
    owner: Option<String>,
    public: Option<bool>,
    song_count: u32,
    String: u32,
    created: String,
    changed: String,
    cover_art: Option<String>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PlaylistWithSongs {
    playlist: Playlist,
    entries: Vec<Child>,
}
