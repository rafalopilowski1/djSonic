use crate::data_structure::child::Child;

use crate::data_structure::playlist::Playlist;

use bytes::Bytes;
use rand::prelude::*;
use reqwest::{Client, StatusCode};
use std::error::Error;
use std::io::{BufReader, Cursor};

use crate::data_structure::album::{AlbumList2, AlbumListType};
use crate::data_structure::child::{NowPlaying, RandomSongs};
use crate::data_structure::playlist::Playlists;
use crate::data_structure::podcast::NewestPodcasts;
use crate::data_structure::search::SearchResult3;
use crate::data_structure::user::User;
use crate::data_structure::{
    artist::{ArtistsID3, Indexes},
    bookmark::Bookmarks,
    directory::Directory,
    genre::Genres,
    music_folder::MusicFolders,
    podcast::Podcasts,
    response::{Error as ResponseError, ResponseValue, SubSonicResponse},
};

use super::traits::CoverArt;

pub(crate) struct SubsonicClient {
    inner_client: Client,
    API_ENDPOINT: String,
    user: String,
    password: String,
    version: Option<String>,
}
impl SubsonicClient {
    pub(crate) async fn new(
        API_ENDPOINT: &str,
        user: &str,
        password: &str,
    ) -> Result<Self, Box<dyn Error>> {
        let mut client_to_init = Self {
            inner_client: Client::new(),
            API_ENDPOINT: API_ENDPOINT.to_owned(),
            user: user.to_owned(),
            password: password.to_owned(),
            version: None,
        };
        client_to_init.init().await?;
        Ok(client_to_init)
    }
    pub(crate) async fn init(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(info) = self.ping().await? {
            self.version = Some(info.version);
        };
        Ok(())
    }
    fn get_auth_token(&self) -> String {
        let random: String = thread_rng().gen::<u64>().to_string();
        let salted_pass = self.password.clone() + &random;
        let hash = format!("{:x}", md5::compute(&salted_pass));
        let mut result = format!("u={0}&t={1}&s={2}", &self.user, &hash, &random,);
        if let Some(version) = &self.version {
            let version_param = format!("&v={}", version);
            result += &version_param;
        }
        result
    }
    async fn get_response_bytes(
        &self,
        path: &str,
        parameters: Option<&str>,
    ) -> Result<Bytes, Box<dyn Error>> {
        let response = self
            .inner_client
            .get(
                self.API_ENDPOINT.clone()
                    + path
                    + "?"
                    + &self.get_auth_token()
                    + parameters.unwrap_or(""),
            )
            .send()
            .await?;
        println!(
            "{}",
            self.API_ENDPOINT.clone()
                + path
                + "?"
                + &self.get_auth_token()
                + parameters.unwrap_or("")
        );
        if response.status() != StatusCode::OK {
            Err(Box::new(ResponseError::new(
                response.status().as_u16(),
                response.status().canonical_reason().unwrap(),
            )))
        } else {
            let response_bytes = response.bytes().await?;
            Ok(response_bytes)
        }
    }
    async fn get_response(
        &self,
        path: &str,
        parameters: Option<&str>,
    ) -> Result<SubSonicResponse, Box<dyn Error>> {
        match self.get_response_bytes(path, parameters).await {
            Ok(response_bytes) => {
                // TODO: Is using in-memory buffer a good idea for response bodies?
                let buf_read = BufReader::new(Cursor::new(response_bytes));
                let mut de = quick_xml::de::Deserializer::from_reader(buf_read);
                let response: Result<SubSonicResponse, _> =
                    serde_path_to_error::deserialize(&mut de);
                match response {
                    Ok(res) => Ok(res),
                    Err(err) => {
                        let path = err.path().to_string();
                        println!("{}", path);
                        Err(Box::new(err))
                    }
                }
            }
            Err(err) => Err(err),
        }
    }
    pub(crate) async fn get_artists(&self) -> Result<Option<ArtistsID3>, Box<dyn Error>> {
        let response = self.get_response("/getArtists", None).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::Artists(artists)) => Ok(Some(artists)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_genres(&self) -> Result<Option<Genres>, Box<dyn Error>> {
        let response = self.get_response("/getGenres", None).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::Genres(genres)) => Ok(Some(genres)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_directory(&self) -> Result<Option<Directory>, Box<dyn Error>> {
        let response = self.get_response("/getMusicDirectory", None).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::Directory(directory)) => Ok(Some(directory)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_music_folders(&self) -> Result<Option<MusicFolders>, Box<dyn Error>> {
        let response = self.get_response("/getMusicFolders", None).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::MusicFolders(music_folders)) => Ok(Some(music_folders)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_indexes(&self) -> Result<Option<Indexes>, Box<dyn Error>> {
        let response = self.get_response("/getIndexes", None).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::Indexes(indexes)) => Ok(Some(indexes)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_bookmarks(&self) -> Result<Option<Bookmarks>, Box<dyn Error>> {
        let response = self.get_response("/getBookmarks", None).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::Bookmarks(bookmarks)) => Ok(Some(bookmarks)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }

    pub(crate) async fn get_podcasts(&self) -> Result<Option<Podcasts>, Box<dyn Error>> {
        let response = self.get_response("/getPodcasts", None).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::Podcasts(podcasts)) => Ok(Some(podcasts)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_now_playing(&self) -> Result<Option<NowPlaying>, Box<dyn Error>> {
        let response = self.get_response("/getNowPlaying", None).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::NowPlaying(nowPlaying)) => Ok(Some(nowPlaying)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_playlists(&self) -> Result<Option<Playlists>, Box<dyn Error>> {
        let response = self.get_response("/getPlaylists", None).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::Playlists(playlists)) => Ok(Some(playlists)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_random_songs(&self) -> Result<Option<RandomSongs>, Box<dyn Error>> {
        let response = self.get_response("/getRandomSongs", None).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::RandomSongs(randomSongs)) => Ok(Some(randomSongs)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_newest_podcasts(
        &self,
    ) -> Result<Option<NewestPodcasts>, Box<dyn Error>> {
        let response = self.get_response("/getNewestPodcasts", None).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::NewestPodcasts(newestPodcasts)) => Ok(Some(newestPodcasts)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_album_list_2(
        &self,
        type_of_album: AlbumListType,
    ) -> Result<Option<AlbumList2>, Box<dyn Error>> {
        let path = "/getAlbumList2".to_owned();
        let parameters = "&type=".to_owned() + &type_of_album.to_string();
        let response = self.get_response(&path, Some(&parameters)).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::AlbumList2(albumList2)) => Ok(Some(albumList2)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn ping(&self) -> Result<Option<SubSonicResponse>, Box<dyn Error>> {
        let path = "/ping".to_owned();
        let response = self.get_response(&path, None).await?;
        Ok(Some(response))
    }
    pub(crate) async fn search3(
        &self,
        query: &str,
    ) -> Result<Option<SearchResult3>, Box<dyn Error>> {
        let path = "/search3".to_owned();
        let parameters = "&query=".to_owned() + &query.to_string();
        let response = self.get_response(&path, Some(&parameters)).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::SearchResult3(search3)) => Ok(Some(search3)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn getUser(&self) -> Result<Option<User>, Box<dyn Error>> {
        let path = "/getUser".to_owned();
        let parameters = "&username=".to_owned() + &self.user;
        let response = self.get_response(&path, Some(&parameters)).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::User(user)) => Ok(Some(user)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_cover_art(
        &self,
        item: impl CoverArt,
    ) -> Result<Option<(Bytes, String)>, Box<dyn Error>> {
        if let Some(cover_art_id) = item.get_cover_art_id() {
            let path = "/getCoverArt".to_owned();
            let parameters = "&id=".to_owned() + cover_art_id;
            let response_bytes = self.get_response_bytes(&path, Some(&parameters)).await?;
            let file_path = cover_art_id.to_owned();

            Ok(Some((response_bytes, file_path)))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn get_playlist(
        &self,
        query_id: u16,
    ) -> Result<Option<Playlist>, Box<dyn Error>> {
        let path = "/getPlaylist".to_owned();
        let parameters = "&id=".to_owned() + &query_id.to_string();
        let response = self.get_response(&path, Some(&parameters)).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::Playlist(playlist)) => Ok(Some(playlist)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }

    pub(crate) async fn get_song(&self, query_id: u16) -> Result<Option<Child>, Box<dyn Error>> {
        let path = "/getSong".to_owned();
        let parameters = "&id=".to_owned() + &query_id.to_string();
        let response = self.get_response(&path, Some(&parameters)).await?;
        let value = response.getValue();
        match value {
            Ok(ResponseValue::Song(child)) => Ok(Some(child)),
            Err(err) => Err(Box::new(err)),
            _ => Ok(None),
        }
    }
}
