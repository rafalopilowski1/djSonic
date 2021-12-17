use std::time::Duration;

use super::child::Child;

pub(crate) struct ScanStatus {
    scanning: bool,
    count: Option<u64>,
}

pub(crate) struct License {
    valid: bool,
    email: String,
    license_expires: Duration,
    trial_expires: Duration,
}
pub(crate) struct Lyrics {
    artist: Option<String>,
    title: Option<String>,
}
pub(crate) struct PlayQueue {
    entries: Vec<Child>,
    current: Option<u32>,
    position: Option<u64>,
    username: String,
    changed: Duration,
    changed_by: String,
}
