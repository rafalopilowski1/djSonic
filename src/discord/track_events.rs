use std::sync::Arc;

use async_trait::async_trait;

use serenity::http::Http;
use songbird::{id::ChannelId, Event, EventContext, EventHandler};

struct TrackUpdateEventHandler {
    channel_id: ChannelId,
    http: Arc<Http>,
}

impl TrackUpdateEventHandler {
    fn new(channel_id: ChannelId, http: Arc<Http>) -> Self {
        Self { channel_id, http }
    }
}

#[async_trait]
impl EventHandler for TrackUpdateEventHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        None
    }
}
