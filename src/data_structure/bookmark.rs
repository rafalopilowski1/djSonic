use std::time::Duration;

use crate::data_structure::child::Child;

pub(crate) struct Bookmark {
    entries: Vec<Child>,
    position: u64,
    username: String,
    comment: Option<String>,
    created: Duration,
    changed: Duration,
}
