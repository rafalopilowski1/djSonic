

use serde::Deserialize;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChatMessage {
    username: String,
    time: String,
    message: String,
}
