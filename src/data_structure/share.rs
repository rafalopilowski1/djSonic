use serde::Deserialize;

use crate::data_structure::child::Child;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Share {
    entries: Vec<Child>,
    id: String,
    url: String,
    description: Option<String>,
    username: String,
    created: String,
    expires: Option<String>,
    last_visited: Option<String>,
    visit_count: u32,
}
