use crate::{
    api::traits::CoverArt,
    data_structure::{album::AlbumID3, child::Child},
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Artist {
    id: String,
    name: String,
    artist_image_url: Option<String>,
    starred: Option<String>,
    user_rating: Option<u32>,    // UserRating: [1,5]
    average_rating: Option<f32>, // AverageRating: [1.0,5.0]
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArtistsID3 {
    #[serde(rename = "$value")]
    artists: Vec<IndexID3>,
    ignored_articles: Option<String>,
}

impl ArtistsID3 {
    pub(crate) fn new(artists: Vec<IndexID3>, ignored_articles: Option<String>) -> Self {
        Self {
            artists,
            ignored_articles,
        }
    }
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IndexID3 {
    #[serde(rename = "$value")]
    artists: Vec<ArtistID3>,
    name: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArtistID3 {
    #[serde(rename = "$value")]
    #[serde(default)]
    albums: Option<Vec<AlbumID3>>,
    id: String,
    pub name: String,
    cover_art: Option<String>,
    artist_image_url: Option<String>,
    album_count: u32,
    starred: Option<String>,
}
impl CoverArt for ArtistID3 {
    fn get_cover_art_id(&self) -> Option<&str> {
        self.cover_art.as_deref()
    }
}
impl ArtistID3 {
    pub(crate) fn get_id(&self) -> &str {
        self.id.as_str()
    }
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArtistInfoBase {
    biography: String,
    music_brainz_id: String,
    last_fm_url: String,
    small_image_url: String,
    medium_image_url: String,
    large_image_url: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArtistInfo {
    base: ArtistInfoBase,
    similar_artists: Vec<Artist>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArtistInfo2 {
    biography: Option<String>,
    music_brainz_id: Option<String>,
    last_fm_url: Option<String>,
    small_image_url: Option<String>,
    medium_image_url: Option<String>,
    large_image_url: Option<String>,
    #[serde(rename = "$value")]
    similar_artists: Option<Vec<ArtistID3>>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Index {
    name: String,
    #[serde(rename = "$value")]
    artists: Vec<Artist>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Indexes {
    last_modified: Option<u64>,
    ignored_articles: String,
    #[serde(rename = "$value")]
    base: Vec<IndexesBase>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum IndexesBase {
    Shorcut(Artist),
    Index(Index),
    Child(Child),
}
