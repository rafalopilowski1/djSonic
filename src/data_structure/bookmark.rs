

use serde::Deserialize;

use crate::data_structure::child::Child;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Bookmark {
    #[serde(rename="$value")]
    entries: Vec<Child>,
    position: u64,
    username: String,
    comment: Option<String>,
    created: String,
    changed: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Bookmarks {
    #[serde(rename="$value")]
    values: Vec<Bookmark>
}
