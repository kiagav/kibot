use std::{env, sync::Arc};

use futures::executor::block_on;
use serenity::{http::Http, model::prelude::ChannelId};

pub trait DiscordAPI {
    fn write_message(&self, message: &str);
}

pub type DynDiscordAPI = Arc<dyn DiscordAPI + Send + Sync>;

pub struct Discord {
    client: Http,
    channel: ChannelId,
}

impl Discord {
    pub fn new() -> Self {
        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let channel = env::var("DISCORD_CHANNEL")
            .expect("Expected a channel in the environment")
            .parse::<u64>()
            .expect("Malformed channel");
        Discord {
            client: Http::new(token.as_str()),
            channel: ChannelId(channel),
        }
    }
}

impl DiscordAPI for Discord {
    fn write_message(&self, message: &str) {
        block_on(async { self.channel.say(&self.client, message).await.unwrap() });
    }
}
