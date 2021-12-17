use crate::data_structure::child::Child;
use std::time::Duration;

pub(crate) enum PodcastStatus {
    New,
    Downloading,
    Completed,
    Error,
    Deleted,
    Skipped,
}

pub(crate) struct PodcastEpisode {
    child: Child,
    stream_id: Option<String>,
    channel_id: String,
    description: Option<String>,
    status: PodcastStatus,
    publish_date: Option<Duration>,
}

pub(crate) struct PodcastChannel {
    episodes: Vec<PodcastEpisode>,
    id: String,
    url: String,
    title: Option<String>,
    description: Option<String>,
    cover_art: Option<String>,
    original_image_url: Option<String>,
    status: PodcastStatus,
    error_message: Option<String>,
}
