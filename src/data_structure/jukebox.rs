use crate::data_structure::child::Child;

pub(crate) struct JukeboxStatus {
    current_index: u32,
    playing: bool,
    gain: f32,
    position: Option<u32>,
}
pub(crate) struct JukeboxPlaylist {
    jukebox_status: JukeboxStatus,
    entries: Vec<Child>,
}
