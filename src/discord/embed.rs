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
        // if let Some((cover_art_bytes, file_path)) = subsonic_client.get_cover_art(self).await? {
        //     let base64String = base64::encode_config(cover_art_bytes, base64::URL_SAFE_NO_PAD);
        //     let url = format!("data:content/type;base64,{}", base64String);
        //     embed = embed.thumbnail(url).to_owned();
        // }

        Ok(embed)
    }
}
