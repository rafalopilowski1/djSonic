pub(crate) struct VideoInfo {
    id: String,
    base: Vec<VideoInfoBase>,
}
pub(crate) enum VideoInfoBase {
    Captions(Captions),
    AudioTrack(AudioTrack),
    Conversion(VideoConversion),
}
pub(crate) struct Captions {
    id: String,
    name: Option<String>,
}
pub(crate) struct AudioTrack {
    id: String,
    name: Option<String>,
    language_code: Option<String>,
}
pub(crate) struct VideoConversion {
    id: String,
    bit_rate: Option<u32>,
    audio_track_id: Option<u32>,
}
