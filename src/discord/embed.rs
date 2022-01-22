use std::error::Error;

use crate::{
    api::subsonic_client::{self, SubsonicClient},
    data_structure::child::Child,
};
use async_trait::async_trait;
use serenity::builder::CreateEmbed;

#[async_trait]
pub trait Embed {
    async fn embed(self, subsonic_client: &SubsonicClient) -> Result<CreateEmbed, Box<dyn Error>>;
}
#[async_trait]
impl Embed for Child {
    async fn embed(self, subsonic_client: &SubsonicClient) -> Result<CreateEmbed, Box<dyn Error>> {
        let mut embed = CreateEmbed::default();
        embed = embed
            .title("Playing")
            .field("Track", self.title.clone(), false)
            .to_owned();

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
