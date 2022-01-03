use serde::Deserialize;

use crate::{api::traits::CoverArt, data_structure::child::Child};
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
    duration: u32,
    created: String,
    changed: String,
    cover_art: Option<String>,
}
impl CoverArt for Playlist {
    fn get_cover_art_id(&self) -> Option<&str> {
        self.cover_art.as_deref()
    }
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PlaylistWithSongs {
    playlist: Playlist,
    entries: Vec<Child>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Playlists {
    #[serde(rename = "$value")]
    #[serde(default)]
    entries: Option<Vec<Playlist>>,
}
