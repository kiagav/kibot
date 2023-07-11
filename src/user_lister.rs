use crate::discord::DynDiscordAPI;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum MsgType {
    Joined,
}

#[derive(Deserialize)]
struct Msg {
    #[serde(rename = "type")]
    msg_type: MsgType,
    #[serde(rename = "userName")]
    username: String,
}

pub struct UserLister {
    pub discord: DynDiscordAPI,
}

impl UserLister {
    pub fn json_message(&self, json: &str) {
        let msg: Msg = serde_json::from_str(json).unwrap();
        match msg.msg_type {
            MsgType::Joined => {
                let message = format!("Users in session: {}", msg.username);
                self.discord.write_message(message.as_str())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::discord::DiscordAPI;

    use super::*;
    use std::sync::{Arc, Mutex};

    struct MemoryDiscord {
        messages: Arc<Mutex<Vec<String>>>,
    }

    impl DiscordAPI for MemoryDiscord {
        fn write_message(&self, message: &str) {
            self.messages.lock().unwrap().push(message.into());
        }
    }

    #[test]
    fn test() {
        let messages = Arc::new(Mutex::new(Vec::new()));
        let mock = Arc::new(MemoryDiscord {
            messages: messages.clone(),
        });
        let lister = UserLister { discord: mock };

        lister.json_message(
            r#"
        {
            "type": "JOINED",
            "userName": "User"
        }
        "#,
        );

        assert_eq!(messages.lock().unwrap()[0], "Users in session: User");
    }
}
