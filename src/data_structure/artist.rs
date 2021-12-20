use std::time::Duration;

use crate::data_structure::{album::AlbumID3, child::Child};
use quick_xml::de::{from_str, DeError};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Artist {
    id: String,
    name: String,
    artist_image_url: Option<String>,
    starred: Option<Duration>,
    user_rating: Option<u32>,    // UserRating: [1,5]
    average_rating: Option<f32>, // AverageRating: [1.0,5.0]
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArtistsID3 {
    #[serde(rename="$value")]
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
    #[serde(rename="$value")]
    artists: Vec<ArtistID3>,
    name: String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArtistID3 {
    id: String,
    name: String,
    cover_art: Option<String>,
    artist_image_url: Option<String>,
    album_count: u32,
    starred: Option<Duration>,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ArtistWithAlbumsID3 {
    artist: ArtistID3,
    albums: Vec<AlbumID3>,
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
    base: ArtistInfoBase,
    similar_artists: Vec<ArtistID3>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Index {
    name: String,
    #[serde(rename="$value")]
    artists: Vec<Artist>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Indexes {
    last_modified: Option<u64>,
    ignored_articles: String,
    #[serde(rename="$value")]
    base: Vec<IndexesBase>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum IndexesBase {
    Shorcut(Artist),
    Index(Index),
    Child(Child),
}
