use crate::data_structure::album::AlbumID3;
use crate::data_structure::artist::{Artist, ArtistID3};
use crate::data_structure::child::Child;
use serde::Deserialize;

#[derive(Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Starred {
    Artist(Artist),
    Album(Child),
    Song(Child),
}
#[derive(Deserialize,Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Starred2 {
    Artist(ArtistID3),
    Album(AlbumID3),
    Song(Box<Child>),
}
