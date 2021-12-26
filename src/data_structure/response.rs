use std::fmt::Display;

use crate::data_structure::{
    album::{AlbumID3, AlbumInfo, AlbumWithSongsID3},
    artist::{ArtistInfo, ArtistInfo2, ArtistWithAlbumsID3, ArtistsID3, Indexes},
    bookmark::Bookmark,
    chat_message::ChatMessage,
    child::{Child, NowPlaying},
    directory::Directory,
    genre::Genre,
    internet_radio::InternetRadioStation,
    jukebox::{JukeboxPlaylist, JukeboxStatus},
    misc::{License, Lyrics, PlayQueue, ScanStatus},
    music_folder::MusicFolder,
    playlist::Playlist,
    podcast::{PodcastChannel, PodcastEpisode},
    search::{SearchResult2, SearchResult3},
    share::Share,
    starred::{Starred, Starred2},
    user::User,
    video::VideoInfo,
};

use crate::data_structure::bookmark::Bookmarks;
use crate::data_structure::genre::Genres;
use crate::data_structure::music_folder::MusicFolders;
use crate::data_structure::playlist::Playlists;
use crate::data_structure::podcast::Podcasts;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename = "kebab-case")]
pub(crate) struct SubSonicResponse {
    #[serde(rename = "$value")]
    value: ResponseValue,
    status: String,  // ResponseStatus: {ok, failed}
    version: String, // Version: regex restriction: `\d+\.\d+\.\d+`
}

impl SubSonicResponse {
    pub(crate) fn getValue(self) -> ResponseValue {
        self.value
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
    Artist(ArtistWithAlbumsID3),
    Album(AlbumWithSongsID3),
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
    AlbumList2(Vec<AlbumID3>),
    RandomSongs(Vec<Child>),
    SongsByGenre(Vec<Child>),
    Lyrics(Lyrics),
    Podcasts(Podcasts),
    NewestPodcasts(Vec<PodcastEpisode>),
    InternetRadioStations(Vec<InternetRadioStation>),
    Bookmarks(Bookmarks),
    PlayQueue(PlayQueue),
    Shares(Vec<Share>),
    Starred(Starred),
    Starred2(Starred2),
    AlbumInfo(AlbumInfo),
    ArtistInfo(ArtistInfo),
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
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
