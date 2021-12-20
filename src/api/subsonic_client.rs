use crate::data_structure::artist::ArtistsID3;
use quick_xml::events::attributes::Attribute;
use rand::prelude::*;
use std::error::Error;
use std::io::{BufRead, BufReader, Cursor};

use crate::data_structure::response::{Error as ResponseError, SubSonicResponse};

use quick_xml::events::{attributes, Event};
use reqwest::{Client, StatusCode};

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
    pub(crate) async fn get_artists(&self) -> Result<SubSonicResponse, Box<dyn Error>> {
        let response = self
            .innerClient
            .get(self.API_ENDPOINT.clone() + "/getArtists" + "?" + &self.get_auth_token())
            .send()
            .await?;
        if response.status() != StatusCode::OK {
            Err(Box::new(ResponseError::new(
                response.status().as_u16(),
                response.status().to_string(),
            )))
        } else {
            let response_bytes = response.bytes().await?;
            let response_clone = response_bytes.clone();
            println!("{}", String::from_utf8(response_clone.to_vec())?);
            // TODO: Is using in-memory buffer a good idea for response bodies?
            let buf_read = BufReader::new(Cursor::new(response_bytes));
            let artists: SubSonicResponse = quick_xml::de::from_reader(buf_read)?;
            Ok(artists)
        }
    }
}
