use rand::prelude::*;
use reqwest::{Client, StatusCode};
use std::error::Error;
use std::io::{BufReader, Cursor};

use crate::data_structure::album::{AlbumList2, AlbumListType};
use crate::data_structure::child::{NowPlaying, RandomSongs};
use crate::data_structure::playlist::Playlists;
use crate::data_structure::podcast::NewestPodcasts;
use crate::data_structure::{
    artist::{ArtistsID3, Indexes},
    bookmark::Bookmarks,
    directory::Directory,
    genre::Genres,
    music_folder::MusicFolders,
    podcast::Podcasts,
    response::{Error as ResponseError, ResponseValue, SubSonicResponse},
};

pub(crate) struct SubsonicClient {
    innerClient: Client,
    API_ENDPOINT: String,
    user: String,
    password: String,
    version: Option<String>,
}
impl SubsonicClient {
    pub(crate) fn new(API_ENDPOINT: &str, user: &str, password: &str) -> Self {
        Self {
            innerClient: Client::new(),
            API_ENDPOINT: API_ENDPOINT.to_owned(),
            user: user.to_owned(),
            password: password.to_owned(),
            version: None,
        }
    }
    pub(crate) async fn init(&mut self) {
        if let Ok(Some(info)) = self.ping().await {
            self.version = Some(info.version);
        } else {
            panic!("Invalid server response!");
        }
    }
    fn get_auth_token(&self) -> String {
        let random: String = thread_rng().gen::<u64>().to_string();
        let salted_pass = self.password.clone() + &random;
        let hash = format!("{:x}", md5::compute(&salted_pass));
        let result = format!("u={0}&t={1}&s={2}", &self.user, &hash, &random);
        result
    }
    async fn get_response(
        &self,
        path: &str,
        parameters: Option<&str>,
    ) -> Result<SubSonicResponse, Box<dyn Error>> {
        let response = self
            .innerClient
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
                response.status().as_str(),
            )))
        } else {
            let response_bytes = response.bytes().await?;

            // TODO: Is using in-memory buffer a good idea for response bodies?
            let buf_read = BufReader::new(Cursor::new(response_bytes));
            let mut de = quick_xml::de::Deserializer::from_reader(buf_read);
            let response: Result<SubSonicResponse, _> = serde_path_to_error::deserialize(&mut de);
            match response {
                Ok(res) => Ok(res),
                Err(err) => {
                    let path = err.path().to_string();
                    println!("{}", path);
                    Err(Box::new(err))
                }
            }
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
}
