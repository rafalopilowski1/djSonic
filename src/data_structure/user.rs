

use serde::Deserialize;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct User {
    folder: Vec<u32>,
    username: String,
    email: Option<String>,
    scrobbling_enabled: bool,
    max_bit_rate: Option<u32>,
    admin_role: bool,
    settings_role: bool,
    download_role: bool,
    upload_role: bool,
    playlist_role: bool,
    cover_art_role: bool,
    comment_role: bool,
    podcast_role: bool,
    stream_bool: bool,
    jukebox_bool: bool,
    share_role: bool,
    video_conversation_role: bool,
    avatar_last_changed: Option<String>,
}
