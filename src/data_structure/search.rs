use crate::data_structure::{
    album::AlbumID3,
    artist::{Artist, ArtistID3},
    child::Child,
};
use serde::Deserialize;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum SearchResult2 {
    Artist(Artist),
    Album(Child),
    Song(Child),
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum SearchResult3Enum {
    Artist(ArtistID3),
    Album(AlbumID3),
    Song(Box<Child>),
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchResult3 {
    #[serde(rename = "$value")]
    #[serde(default)]
    value: Option<SearchResult3Enum>,
}
