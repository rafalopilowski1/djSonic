use serde::Deserialize;

use crate::data_structure::child::Child;
use std::time::Duration;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Share {
    entries: Vec<Child>,
    id: String,
    url: String,
    description: Option<String>,
    username: String,
    created: Duration,
    expires: Option<Duration>,
    last_visited: Option<Duration>,
    visit_count: u32,
}
