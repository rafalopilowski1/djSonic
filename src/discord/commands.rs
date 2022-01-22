use serenity::{
    async_trait,
    model::{
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommandInteractionDataOptionValue, ApplicationCommandOptionType,
            },
            InteractionResponseType,
        },
        prelude::{Interaction, Ready},
    },
    prelude::*,
};
use tokio::runtime::Handle;

use crate::api::subsonic_client::SubsonicClient;

use super::embed::Embed;

pub struct Handler {
    pub subsonic_client: SubsonicClient,
}
impl Handler {
    pub fn new(subsonic_client: SubsonicClient) -> Self {
        Self { subsonic_client }
    }
}
#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "play" => {
                    let track_id = command
                        .data
                        .options
                        .get(0)
                        .expect("Expected track ID v1")
                        .resolved
                        .as_ref()
                        .expect("Expected track ID v2");
                    if let ApplicationCommandInteractionDataOptionValue::Integer(track_id_to_play) =
                        track_id
                    {
                        match self
                            .subsonic_client
                            .get_song(*track_id_to_play as u16)
                            .await
                            .ok()
                        {
                            Some(response) => response,
                            None => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(child) = content.as_ref() {
                let guild_id = command.guild_id.unwrap();
                let manager = songbird::get(&ctx)
                    .await
                    .expect("Songbird init failure!")
                    .clone();
                let member = command.member.as_ref().unwrap();

                let _handler = manager.join(guild_id, 920001255445770264).await;
                if let Some(handler_lock) = manager.get(guild_id) {
                    let mut handler = handler_lock.lock().await;

                    let stream_url = self
                        .subsonic_client
                        .stream_url(child)
                        .await
                        .expect("Stream URL not avaliable!");
                    let source = songbird::ffmpeg(stream_url.unwrap()).await.ok().unwrap();
                    let track_handle = handler.play_only_source(source);
                    track_handle.play();
                }
            }
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|messange| {
                            if let Some(child) = content {
                                messange.add_embed(
                                    tokio::task::block_in_place(|| {
                                        Handle::current().block_on(async move {
                                            child.embed(&self.subsonic_client).await
                                        })
                                    })
                                    .expect("Bad parsing!"),
                                );
                                messange
                            } else {
                                messange.create_embed(|embed| {
                                    embed.color(0).title("Error").field(
                                        "Origin",
                                        "Track not found!",
                                        false,
                                    )
                                })
                            }
                        })
                })
                .await
            {
                eprintln!("Cannot respond to slash message: {}", why);
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let guild_id = GuildId(
            dotenv::var("GUILD_ID")
                .expect("GUILD_ID not fount in dotfile!")
                .parse()
                .expect("Invalid guild ID!"),
        );
        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("play")
                    .description("Play given ID")
                    .create_option(|option| {
                        option
                            .name("id")
                            .description("ID to play")
                            .required(true)
                            .kind(ApplicationCommandOptionType::Integer)
                    })
            })
        })
        .await;
        println!("Created guild slash commands: {:#?}", commands);
    }
}
