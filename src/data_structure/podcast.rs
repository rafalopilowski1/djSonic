use serde::Deserialize;

use crate::data_structure::child::Child;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum PodcastStatus {
    New,
    Downloading,
    Completed,
    Error,
    Deleted,
    Skipped,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PodcastEpisode {
    #[serde(flatten)]
    child: Child,

    stream_id: Option<String>,
    channel_id: String,
    description: Option<String>,
    status: PodcastStatus,
    publish_date: Option<String>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PodcastChannel {
    #[serde(rename = "$value")]
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
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Podcasts {
    #[serde(rename = "$value")]
    values: Vec<PodcastChannel>,
}
