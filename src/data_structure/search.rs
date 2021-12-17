use crate::data_structure::{
    album::AlbumID3,
    artist::{Artist, ArtistID3},
    child::Child,
};

pub(crate) enum SearchResult2 {
    Artist(Artist),
    Album(Child),
    Song(Child),
}
pub(crate) enum SearchResult3 {
    Artist(ArtistID3),
    Album(AlbumID3),
    Song(Box<Child>),
}
