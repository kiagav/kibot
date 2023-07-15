use dotenv::dotenv;
use serenity::{
    async_trait,
    http::Http,
    model::prelude::{ChannelId, MessageId},
};
use std::{
    env,
    sync::{Arc, Mutex},
};

#[async_trait]
pub trait DiscordAPI {
    async fn write_message(&self, message: &str);
    async fn clear_all_messages(&self);
}

pub type DynDiscordAPI = Arc<dyn DiscordAPI + Send + Sync>;

pub struct Discord {
    client: Arc<Http>,
    channel: Arc<ChannelId>,
    sent_message_ids: Mutex<Vec<MessageId>>,
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
            sent_message_ids: Mutex::new(Vec::new()),
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
        let msg_id = self
            .channel
            .say(self.client.clone(), message)
            .await
            .unwrap()
            .id;
        self.sent_message_ids.lock().unwrap().push(msg_id);
    }

    async fn clear_all_messages(&self) {
        let ids = self.sent_message_ids.lock().unwrap().clone();
        for id in ids.iter() {
            let _ = self.channel.delete_message(self.client.clone(), id).await;
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::{Discord, DiscordAPI};

    #[tokio::test]
    #[ignore]
    async fn test_connection() {
        let discord = Discord::new();
        let _ = discord.write_message("test1").await;
        let _ = discord.write_message("test2").await;
        let _ = discord.write_message("test3").await;
        std::thread::sleep(Duration::from_secs(3));
        let _ = discord.clear_all_messages().await;
    }
}
