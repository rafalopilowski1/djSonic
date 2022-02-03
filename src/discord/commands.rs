use std::{any::Any, sync::Arc};

use serenity::{
    async_trait,
    builder::CreateEmbed,
    model::{
        channel::ChannelType,
        id::{ChannelId, GuildId},
        interactions::{
            application_command::{
                ApplicationCommandInteraction, ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            InteractionResponseType,
        },
        prelude::{Interaction, Ready},
    },
    prelude::*,
};
use songbird::Songbird;
use tokio::runtime::Handle;

use crate::{
    api::subsonic_client::SubsonicClient,
    data_structure::{child::Child, search::SearchResult3Enum},
};

use super::embed::{Embed, EmbedType};
pub struct Handler {
    pub subsonic_client: Arc<SubsonicClient>,
}
impl Handler {
    pub fn new(subsonic_client: SubsonicClient) -> Self {
        Self {
            subsonic_client: Arc::new(subsonic_client),
        }
    }
}
#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let guild_id = command.guild_id.unwrap();
            let manager = songbird::get(&ctx)
                .await
                .expect("Songbird init failure!")
                .clone();
            match command.data.name.as_str() {
                "play" => {
                    let content = get_content_to_play(&command, &self.subsonic_client).await;
                    play_on_discord(
                        &command,
                        &ctx,
                        content,
                        &self.subsonic_client,
                        Arc::clone(&manager),
                        guild_id,
                    )
                    .await;
                }
                "stop" => {
                    stop_on_discord(Arc::clone(&manager), guild_id, &command, &ctx).await;
                }
                "search" => {
                    let query = get_query_string_value(&command, &self.subsonic_client).await;
                    display_search_query(query.as_deref(), &command, &ctx, &self.subsonic_client)
                        .await;
                }
                _ => {}
            };
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let guild_id = GuildId(
            std::env::var("GUILD_ID")
                .expect("GUILD_ID not fount in dotfile!")
                .parse()
                .expect("Invalid guild ID!"),
        );
        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
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
                .create_application_command(|command| {
                    command
                        .name("stop")
                        .description("Stop playing and kich the bot out of voice channel")
                })
                .create_application_command(|command| {
                    command
                        .name("search")
                        .description("Search")
                        .create_option(|option| {
                            option
                                .name("query")
                                .description("Value to search for")
                                .required(true)
                                .kind(ApplicationCommandOptionType::String)
                        })
                })
        })
        .await;
        println!("Created guild slash commands: {:#?}", commands);
    }
}

pub async fn get_content_to_play(
    command: &ApplicationCommandInteraction,
    subsonic_client: &SubsonicClient,
) -> Option<Child> {
    let track_id = command
        .data
        .options
        .get(0)
        .expect("Expected track ID v1")
        .resolved
        .as_ref()
        .expect("Expected track ID v2");
    if let ApplicationCommandInteractionDataOptionValue::Integer(track_id_to_play) = track_id {
        match subsonic_client
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
pub async fn play_on_discord(
    command: &ApplicationCommandInteraction,
    ctx: &Context,
    content: Option<Child>,
    subsonic_client: &SubsonicClient,
    manager: Arc<Songbird>,
    guild_id: GuildId,
) {
    command
        .create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
        })
        .await
        .expect("Failure posting!");
    if let Some(child) = content.as_ref() {
        let mut channelId: Option<ChannelId> = None;
        let user = &command.user;
        let channels = ctx
            .http
            .get_channels(*guild_id.as_u64())
            .await
            .expect("Channels failed!");
        for channel in channels {
            if channel.kind == ChannelType::Voice
                && channel
                    .members(&ctx.cache)
                    .await
                    .expect("Channel members failed!")
                    .iter()
                    .any(|member| member.user.id == user.id)
            {
                channelId = Some(channel.id);
            }
        }
        if let Some(channel_id) = channelId {
            let (handler, result) = manager.join(guild_id, channel_id).await;
            if let Err(err) = result {
                println!("{:#?}", err);
            } else {
                let mut handler_unlocked = handler.lock().await;
                let stream_url = subsonic_client
                    .stream_url(child)
                    .await
                    .expect("Stream URL not avaliable!");
                let source = songbird::input::Restartable::ffmpeg(stream_url.unwrap(), false)
                    .await
                    .ok()
                    .unwrap();
                let (audio, trackHandle) = songbird::tracks::create_player(source.into());
                handler_unlocked.play_only(audio);
            }
            command
                .edit_original_interaction_response(&ctx.http, |messange| {
                    if let Some(child) = content {
                        messange.add_embed(
                            tokio::task::block_in_place(|| {
                                Handle::current().block_on(async move {
                                    child.embed(&subsonic_client, EmbedType::Playing).await
                                })
                            })
                            .expect("Bad parsing!"),
                        );
                        messange
                    } else {
                        messange.add_embed(
                            CreateEmbed::default()
                                .color(0)
                                .title("Error")
                                .field("Origin", "Track not found!", false)
                                .to_owned(),
                        );
                        messange
                    }
                })
                .await
                .expect("Edit failure!");
        } else {
            command
                .edit_original_interaction_response(&ctx.http, |messange| {
                    messange.add_embed(
                        CreateEmbed::default()
                            .color(0)
                            .title("Error")
                            .field("Origin", "Not connected to any voice channel!", false)
                            .to_owned(),
                    );
                    messange
                })
                .await
                .expect("Voice channel not found failure!");
        }
    }
}
pub async fn stop_on_discord(
    manager: Arc<Songbird>,
    guild_id: GuildId,
    command: &ApplicationCommandInteraction,
    ctx: &Context,
) {
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        handler.stop();
        handler.leave().await.expect("Leave failure!");
    }
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|messange| {
                    messange.add_embed(CreateEmbed::default().title("Stopped!").to_owned())
                })
        })
        .await
        .expect("Pong failure!");
}
pub async fn get_query_string_value(
    command: &ApplicationCommandInteraction,
    subsonic_client: &SubsonicClient,
) -> Option<Vec<SearchResult3Enum>> {
    let query_string = command
        .data
        .options
        .get(0)
        .expect("Expected query string v1")
        .resolved
        .as_ref()
        .expect("Expected query string v2");
    if let ApplicationCommandInteractionDataOptionValue::String(string) = query_string {
        if let Ok(Some(search_result)) = subsonic_client.search3(string).await {
            search_result.getValues()
        } else {
            None
        }
    } else {
        None
    }
}
pub async fn display_search_query(
    values: Option<&[SearchResult3Enum]>,
    command: &ApplicationCommandInteraction,
    ctx: &Context,
    subsonic_client: &SubsonicClient,
) {
    command
        .create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::DeferredChannelMessageWithSource)
        })
        .await
        .expect("Failure posting!");
    if let Some(values) = values {
        command
            .edit_original_interaction_response(&ctx.http, |messange| {
                messange.add_embed(
                    CreateEmbed::default()
                        .color(0)
                        .title("Search results:")
                        .to_owned(),
                );
                for search_value in values.iter() {
                    messange.add_embed(
                        tokio::task::block_in_place(|| {
                            Handle::current().block_on(async move {
                                match search_value {
                                    SearchResult3Enum::Artist(artist) => {
                                        artist.embed(subsonic_client, EmbedType::SearchResult).await
                                    }
                                    SearchResult3Enum::Album(album) => {
                                        album.embed(subsonic_client, EmbedType::SearchResult).await
                                    }
                                    SearchResult3Enum::Song(song) => {
                                        song.embed(subsonic_client, EmbedType::SearchResult).await
                                    }
                                }
                            })
                        })
                        .expect("Bad parsing!"),
                    );
                }
                messange
            })
            .await
            .expect("Search results failure!");
    } else {
        command
            .edit_original_interaction_response(&ctx.http, |messange| {
                messange.add_embed(
                    CreateEmbed::default()
                        .color(0)
                        .title("No results found")
                        .to_owned(),
                )
            })
            .await
            .expect("Not results found failure!");
    }
}
