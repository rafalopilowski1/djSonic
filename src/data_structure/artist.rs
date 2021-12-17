use std::time::Duration;

use crate::data_structure::{album::AlbumID3, child::Child};

pub(crate) struct Artist {
    id: String,
    name: String,
    artist_image_url: Option<String>,
    starred: Option<Duration>,
    user_rating: Option<u32>,    // UserRating: [1,5]
    average_rating: Option<f32>, // AverageRating: [1.0,5.0]
}
pub(crate) struct ArtistsID3 {
    artists: Vec<IndexID3>,
    ignored_articles: String,
}
pub(crate) struct IndexID3 {
    artists: Vec<ArtistID3>,
    name: String,
}
pub(crate) struct ArtistID3 {
    id: String,
    name: String,
    cover_art: Option<String>,
    artist_image_url: Option<String>,
    album_count: u32,
    starred: Option<Duration>,
}
pub(crate) struct ArtistWithAlbumsID3 {
    artist: ArtistID3,
    albums: Vec<AlbumID3>,
}
pub(crate) struct ArtistInfoBase {
    biography: String,
    music_brainz_id: String,
    last_fm_url: String,
    small_image_url: String,
    medium_image_url: String,
    large_image_url: String,
}

pub(crate) struct ArtistInfo {
    base: ArtistInfoBase,
    similar_artists: Vec<Artist>,
}
pub(crate) struct ArtistInfo2 {
    base: ArtistInfoBase,
    similar_artists: Vec<ArtistID3>,
}

pub(crate) struct Index {
    name: String,
    artists: Vec<Artist>,
}
pub(crate) struct Indexes {
    last_modified: u64,
    ignored_articles: String,
    base: IndexesBase,
}
pub(crate) enum IndexesBase {
    Shorcut(Artist),
    Index(Index),
    Child(Box<Child>),
}
