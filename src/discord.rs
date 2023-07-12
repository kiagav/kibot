use dotenv::dotenv;
use serenity::{async_trait, http::Http, model::prelude::ChannelId};
use std::{env, sync::Arc};

#[async_trait]
pub trait DiscordAPI {
    async fn write_message(&self, message: &str);
}

pub type DynDiscordAPI = Arc<dyn DiscordAPI + Send + Sync>;

pub struct Discord {
    client: Arc<Http>,
    channel: Arc<ChannelId>,
}

impl Discord {
    pub fn new() -> Self {
        dotenv().ok();
        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let channel = env::var("DISCORD_CHANNEL")
            .expect("Expected a channel in the environment")
            .parse::<u64>()
            .expect("Malformed channel");
        Discord {
            client: Arc::new(Http::new(token.as_str())),
            channel: Arc::new(ChannelId(channel)),
        }
    }
}

impl Default for Discord {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DiscordAPI for Discord {
    async fn write_message(&self, message: &str) {
        println!(
            "Sending to channel {}, message: {}",
            self.channel.0, message
        );
        self.channel
            .say(self.client.clone(), message)
            .await
            .unwrap();
    }
}

#[cfg(test)]

mod test {
    use super::{Discord, DiscordAPI};

    #[tokio::test]
    #[ignore]
    async fn test_connection() {
        let discord = Discord::new();
        let _ = discord.write_message("test").await;
    }
}
