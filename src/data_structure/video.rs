use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VideoInfo {
    id: String,
    base: Vec<VideoInfoBase>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum VideoInfoBase {
    Captions(Captions),
    AudioTrack(AudioTrack),
    Conversion(VideoConversion),
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Captions {
    id: String,
    name: Option<String>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AudioTrack {
    id: String,
    name: Option<String>,
    language_code: Option<String>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VideoConversion {
    id: String,
    bit_rate: Option<u32>,
    audio_track_id: Option<u32>,
}
