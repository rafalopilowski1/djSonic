use crate::{
    api::traits::CoverArt,
    data_structure::{
        album::AlbumID3,
        artist::{Artist, ArtistID3},
        child::Child,
    },
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
pub enum SearchResult3Enum {
    Artist(ArtistID3),
    Album(AlbumID3),
    Song(Box<Child>),
}
impl CoverArt for &SearchResult3Enum {
    fn get_cover_art_id(&self) -> Option<&str> {
        match self {
            SearchResult3Enum::Artist(artist) => artist.get_cover_art_id(),
            SearchResult3Enum::Album(album) => album.get_cover_art_id(),
            SearchResult3Enum::Song(track) => track.get_cover_art_id(),
        }
    }
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SearchResult3 {
    #[serde(rename = "$value")]
    #[serde(default)]
    value: Option<Vec<SearchResult3Enum>>,
}
impl SearchResult3 {
    pub fn getValues(self) -> Option<Vec<SearchResult3Enum>> {
        self.value
    }
}
