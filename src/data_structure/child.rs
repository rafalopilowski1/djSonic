use serde::Deserialize;

// Unnessary boilerplate cause by [this issue #2117](https://github.com/serde-rs/serde/issues/2117).

use serde_with::{serde_as, DisplayFromStr};

use crate::api::traits::{CoverArt, Streamable};
#[serde_as]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Child {
    id: String,
    parent: Option<String>,

    #[serde_as(as = "DisplayFromStr")]
    is_dir: bool,

    title: String,
    album: Option<String>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    track: Option<u32>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    year: Option<u32>,

    genre: Option<String>,
    cover_art: Option<String>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    size: Option<u64>,

    content_type: Option<String>,
    suffix: Option<String>,
    transcoded_content_type: Option<String>,
    transcoded_suffix: Option<String>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    duration: Option<u32>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    bit_rate: Option<u32>,

    path: Option<String>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    is_video: Option<bool>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    user_rating: Option<u32>, // UserRating: [1,5]

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    average_rating: Option<f32>, // AverageRating: [1.0,5.0]

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    play_count: Option<u64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    disc_number: Option<u32>,

    created: Option<String>,
    starred: Option<String>,
    album_id: Option<String>,
    artist_id: Option<String>,
    media_type: Option<MediaType>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    bookmark_position: Option<u64>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    original_width: Option<u32>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    original_height: Option<u32>,
}
impl CoverArt for Child {
    fn get_cover_art_id(&self) -> Option<&str> {
        self.cover_art.as_deref()
    }
}
impl Streamable for Child {
    fn get_id(&self) -> &str {
        self.id.as_ref()
    }
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
enum MediaType {
    Music,
    Podcast,
    Audiobook,
    Video,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NowPlaying {
    #[serde(rename = "$value")]
    entries: Vec<NowPlayingEntry>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NowPlayingEntry {
    child: Child,
    username: String,
    minutes_ago: u32,
    player_id: u32,
    player_name: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RandomSongs {
    #[serde(rename = "$value")]
    entries: Vec<Child>,
}
