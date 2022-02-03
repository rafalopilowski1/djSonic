use std::error::Error;

use crate::{
    api::subsonic_client::{self, SubsonicClient},
    data_structure::{album::AlbumID3, artist::ArtistID3, child::Child},
};
use async_trait::async_trait;
use serenity::builder::CreateEmbed;

pub enum EmbedType {
    Playing,
    SearchResult,
}

#[async_trait]
pub trait Embed {
    async fn embed(
        &self,
        subsonic_client: &SubsonicClient,
        embedType: EmbedType,
    ) -> Result<CreateEmbed, Box<dyn Error>>;
}
#[async_trait]
impl Embed for Child {
    async fn embed(
        &self,
        subsonic_client: &SubsonicClient,
        embedType: EmbedType,
    ) -> Result<CreateEmbed, Box<dyn Error>> {
        let mut embed = CreateEmbed::default();
        match embedType {
            EmbedType::Playing => {
                embed = embed
                    .title("Playing")
                    .field("Track", self.title.clone(), false)
                    .to_owned();
            }
            EmbedType::SearchResult => {
                embed = embed.title(self.title.clone()).to_owned();
            }
        }

        if let Some(ref albumName) = self.album {
            embed = embed
                .field("Album", format!("**{}**", albumName), true)
                .to_owned();
        }
        if let Some(ref artist_id) = self.artist_id {
            let artistIdU16: u16 = artist_id.parse()?;
            if let Some(artist) = subsonic_client.get_artist(artistIdU16).await? {
                embed = embed
                    .field("Artist", format!("*{}*", artist.name), true)
                    .to_owned();
            };
        }
        if let Some(url_to_cover) = subsonic_client.get_cover_art_url(self).await? {
            embed = embed.thumbnail(url_to_cover).to_owned();
        }

        Ok(embed)
    }
}

#[async_trait]
impl Embed for ArtistID3 {
    async fn embed(
        &self,
        subsonic_client: &SubsonicClient,
        _embedType: EmbedType,
    ) -> Result<CreateEmbed, Box<dyn Error>> {
        let mut embed = CreateEmbed::default();
        embed = embed
            .title(self.name.clone())
            .field("Album count", &self.album_count, true)
            .to_owned();
        if let Some(url_to_cover) = subsonic_client.get_cover_art_url(self).await? {
            embed = embed.thumbnail(url_to_cover).to_owned();
        } else if let Some(url_to_cover) = &self.artist_image_url {
            embed = embed.thumbnail(url_to_cover).to_owned();
        }
        Ok(embed)
    }
}

#[async_trait]
impl Embed for AlbumID3 {
    async fn embed(
        &self,
        subsonic_client: &SubsonicClient,
        embedType: EmbedType,
    ) -> Result<CreateEmbed, Box<dyn Error>> {
        let mut embed = CreateEmbed::default();
        embed = embed
            .title(self.name.clone())
            .field("Track count", &self.song_count, false)
            .to_owned();
        if let Some(url_to_cover) = subsonic_client.get_cover_art_url(self).await? {
            embed = embed.thumbnail(url_to_cover).to_owned();
        }
        if let Some(ref artist) = self.artist {
            embed = embed.field("Artist", artist, false).to_owned();
        }
        Ok(embed)
    }
}
