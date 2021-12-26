use rand::prelude::*;
use reqwest::{Client, StatusCode};
use std::error::Error;
use std::io::{BufReader, Cursor};

use crate::data_structure::child::NowPlaying;
use crate::data_structure::playlist::Playlists;
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
}
impl SubsonicClient {
    pub(crate) fn new(API_ENDPOINT: &str, user: &str, password: &str) -> Self {
        Self {
            innerClient: Client::new(),
            API_ENDPOINT: API_ENDPOINT.to_owned(),
            user: user.to_owned(),
            password: password.to_owned(),
        }
    }
    fn get_auth_token(&self) -> String {
        let random: String = thread_rng().gen::<u64>().to_string();
        let salted_pass = self.password.clone() + &random;
        let hash = format!("{:x}", md5::compute(&salted_pass));
        let result = format!("u={0}&t={1}&s={2}", &self.user, &hash, &random);
        result
    }
    async fn get_response(&self, path: &str) -> Result<SubSonicResponse, Box<dyn Error>> {
        let response = self
            .innerClient
            .get(self.API_ENDPOINT.clone() + path + "?" + &self.get_auth_token())
            .send()
            .await?;
        println!(
            "{}",
            self.API_ENDPOINT.clone() + path + "?" + &self.get_auth_token()
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
        let response = self.get_response("/getArtists").await?;
        let value = response.getValue();
        match value {
            ResponseValue::Artists(artists) => Ok(Some(artists)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_genres(&self) -> Result<Option<Genres>, Box<dyn Error>> {
        let response = self.get_response("/getGenres").await?;
        let value = response.getValue();
        match value {
            ResponseValue::Genres(genres) => Ok(Some(genres)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_directory(&self) -> Result<Option<Directory>, Box<dyn Error>> {
        let response = self.get_response("/getMusicDirectory").await?;
        let value = response.getValue();
        match value {
            ResponseValue::Directory(directory) => Ok(Some(directory)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_music_folders(&self) -> Result<Option<MusicFolders>, Box<dyn Error>> {
        let response = self.get_response("/getMusicFolders").await?;
        let value = response.getValue();
        match value {
            ResponseValue::MusicFolders(music_folders) => Ok(Some(music_folders)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_indexes(&self) -> Result<Option<Indexes>, Box<dyn Error>> {
        let response = self.get_response("/getIndexes").await?;
        let value = response.getValue();
        match value {
            ResponseValue::Indexes(indexes) => Ok(Some(indexes)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_bookmarks(&self) -> Result<Option<Bookmarks>, Box<dyn Error>> {
        let response = self.get_response("/getBookmarks").await?;
        let value = response.getValue();
        match value {
            ResponseValue::Bookmarks(bookmarks) => Ok(Some(bookmarks)),
            _ => Ok(None),
        }
    }

    pub(crate) async fn get_podcasts(&self) -> Result<Option<Podcasts>, Box<dyn Error>> {
        let response = self.get_response("/getPodcasts").await?;
        let value = response.getValue();
        match value {
            ResponseValue::Podcasts(podcasts) => Ok(Some(podcasts)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_now_playing(&self) -> Result<Option<NowPlaying>, Box<dyn Error>> {
        let response = self.get_response("/getNowPlaying").await?;
        let value = response.getValue();
        match value {
            ResponseValue::NowPlaying(nowPlaying) => Ok(Some(nowPlaying)),
            _ => Ok(None),
        }
    }
    pub(crate) async fn get_playlists(&self) -> Result<Option<Playlists>, Box<dyn Error>> {
        let response = self.get_response("/getPlaylists").await?;
        let value = response.getValue();
        match value {
            ResponseValue::Playlists(playlists) => Ok(Some(playlists)),
            _ => Ok(None),
        }
    }
}
