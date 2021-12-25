

use serde::Deserialize;

use super::child::Child;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ScanStatus {
    scanning: bool,
    count: Option<u64>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct License {
    valid: bool,
    email: String,
    license_expires: String,
    trial_expires: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Lyrics {
    artist: Option<String>,
    title: Option<String>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PlayQueue {
    entries: Vec<Child>,
    current: Option<u32>,
    position: Option<u64>,
    username: String,
    changed: String,
    changed_by: String,
}
