
use std::time::Duration;

pub(crate) struct ChatMessage {
    username: String,
    time: Duration,
    message: String,
}
