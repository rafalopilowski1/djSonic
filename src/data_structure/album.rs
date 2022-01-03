use std::fmt::Display;

use serde::Deserialize;

use crate::api::traits::CoverArt;
use crate::data_structure::child::Child;
use crate::data_structure::genre::Genre;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AlbumID3 {
    id: String,
    name: String,
    artist: Option<String>,
    artist_id: Option<String>,
    cover_art: Option<String>,
    song_count: u32,
    duration: Option<u32>,
    play_count: Option<u64>,
    created: String,
    starred: Option<String>,
    year: Option<u32>,
    genre: Option<String>,
}
impl CoverArt for AlbumID3 {
    fn get_cover_art_id(&self) -> Option<&str> {
        self.cover_art.as_deref()
    }
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AlbumList2 {
    #[serde(rename = "$value")]
    entries: Vec<AlbumID3>,
}

pub(crate) enum AlbumListType {
    Random,
    Newest,
    Frequent,
    Recent,
    Starred,
    AlphabeticalByName,
    AlphabeticalByArtist,
    ByYear(u32, u32),
    ByGenre(Genre),
}

impl Display for AlbumListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AlbumListType::Random => "random".to_owned(),
            AlbumListType::Newest => "newest".to_owned(),
            AlbumListType::Frequent => "frequent".to_owned(),
            AlbumListType::Recent => "recent".to_owned(),
            AlbumListType::Starred => "starred".to_owned(),
            AlbumListType::AlphabeticalByName => "alphabeticalByName".to_owned(),
            AlbumListType::AlphabeticalByArtist => "alphabeticalByArtist".to_owned(),
            AlbumListType::ByYear(fromYear, toYear) => {
                format!("byYear&fromYear={0}&toYear={1}", fromYear, toYear)
            }

            AlbumListType::ByGenre(genre) => format!("byGenre&genre={}", genre.getName()),
        };
        write!(f, "{}", str)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AlbumWithSongsID3 {
    album: AlbumID3,
    songs: Vec<Child>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AlbumInfo {
    notes: Option<String>,
    music_brainz_id: Option<String>,
    last_fm_url: Option<String>,
    small_image_url: Option<String>,
    medium_image_url: Option<String>,
    large_image_url: Option<String>,
}
