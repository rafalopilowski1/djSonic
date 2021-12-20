use serde::Deserialize;

use crate::data_structure::child::Child;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JukeboxStatus {
    current_index: u32,
    playing: bool,
    gain: f32,
    position: Option<u32>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JukeboxPlaylist {
    jukebox_status: JukeboxStatus,
    entries: Vec<Child>,
}
