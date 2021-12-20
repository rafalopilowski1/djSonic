use std::time::Duration;

use serde::Deserialize;

use crate::data_structure::child::Child;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Bookmark {
    entries: Vec<Child>,
    position: u64,
    username: String,
    comment: Option<String>,
    created: Duration,
    changed: Duration,
}
