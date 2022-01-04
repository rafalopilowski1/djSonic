use crate::data_structure::artist::ArtistID3;
use std::fmt::Display;

use crate::data_structure::{
    album::{AlbumID3, AlbumInfo, AlbumList2},
    artist::{ArtistInfo, ArtistInfo2, ArtistsID3, Indexes},
    bookmark::Bookmarks,
    chat_message::ChatMessage,
    child::{Child, NowPlaying, RandomSongs},
    directory::Directory,
    genre::Genres,
    internet_radio::InternetRadioStation,
    jukebox::{JukeboxPlaylist, JukeboxStatus},
    misc::{License, Lyrics, PlayQueue, ScanStatus},
    music_folder::MusicFolders,
    playlist::{Playlist, Playlists},
    podcast::{NewestPodcasts, Podcasts},
    search::{SearchResult2, SearchResult3Enum},
    share::Share,
    starred::{Starred, Starred2},
    user::User,
    video::VideoInfo,
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename = "kebab-case")]
pub(crate) struct SubSonicResponse {
    #[serde(rename = "$value")]
    #[serde(default)]
    value: Option<ResponseValue>,
    status: String,      // ResponseStatus: {ok, failed}
    pub version: String, // Version: regex restriction: `\d+\.\d+\.\d+`
}

use crate::data_structure::response::Error as ResponseError;

use super::search::SearchResult3;

impl SubSonicResponse {
    pub(crate) fn getValue(self) -> Result<ResponseValue, ResponseError> {
        if let Some(data) = self.value {
            match data {
                ResponseValue::Error(err) => Err(err),
                _ => Ok(data),
            }
        } else {
            Err(ResponseError::new(0, "Empty / invalid body!"))
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ResponseValue {
    MusicFolders(MusicFolders),
    Indexes(Indexes),
    Directory(Directory),
    Genres(Genres),
    Artists(ArtistsID3),
    Artist(ArtistID3),
    Album(AlbumID3),
    Song(Child),
    Videos(Vec<Child>),
    VideoInfo(VideoInfo),
    NowPlaying(NowPlaying),
    // searchResult,
    SearchResult2(SearchResult2),
    SearchResult3(SearchResult3),
    Playlists(Playlists),
    Playlist(Playlist),
    JukeboxStatus(JukeboxStatus),
    JukeboxPlaylist(JukeboxPlaylist),
    License(License),
    Users(Vec<User>),
    User(User),
    ChatMessages(Vec<ChatMessage>),
    AlbumList(Vec<Child>),
    AlbumList2(AlbumList2),
    RandomSongs(RandomSongs),
    SongsByGenre(Vec<Child>),
    Lyrics(Lyrics),
    Podcasts(Podcasts),
    NewestPodcasts(NewestPodcasts),
    InternetRadioStations(Vec<InternetRadioStation>),
    Bookmarks(Bookmarks),
    PlayQueue(PlayQueue),
    Shares(Vec<Share>),
    Starred(Starred),
    Starred2(Starred2),
    AlbumInfo(AlbumInfo),
    ArtistInfo(ArtistInfo),
    #[serde(rename = "artist-info2")]
    ArtistInfo2(ArtistInfo2),
    SimilarSongs(Vec<Child>),
    SimilarSongs2(Vec<Child>),
    TopSongs(Vec<Child>),
    ScanStatus(ScanStatus),
    Error(Error),
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Error {
    code: u16,
    message: String,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {0} - {1}", self.code, self.message)
    }
}

impl Error {
    pub(crate) fn new(code: u16, message: &str) -> Self {
        Self {
            code,
            message: message.to_owned(),
        }
    }
    pub(crate) fn getCode(self) -> u16 {
        self.code
    }
    pub(crate) fn getMessage(&self) -> &str {
        &self.message
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
