use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

use crate::discord::DynDiscordAPI;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum MsgType {
    Joined,
    Left,
}

#[derive(Deserialize)]
struct Msg {
    #[serde(rename = "type")]
    msg_type: MsgType,
    #[serde(rename = "userName")]
    username: String,
}

pub struct UserLister {
    users: Arc<Mutex<Vec<String>>>,
    pub discord: DynDiscordAPI,
}

impl UserLister {
    pub fn new(discord: DynDiscordAPI) -> Self {
        UserLister {
            users: Arc::new(Mutex::new(Vec::new())),
            discord,
        }
    }
    pub async fn json_message(&self, json: &str) {
        let msg: Msg = serde_json::from_str(json).unwrap();
        let mut users = self.users.lock().await;
        match msg.msg_type {
            MsgType::Joined => {
                if !users.contains(&msg.username) {
                    users.push(msg.username);
                }
                self.print_users_in_session(users).await;
            }
            MsgType::Left => {
                users.retain(|name| *name != msg.username);
                self.print_users_in_session(users).await;
            }
        }
    }

    async fn print_users_in_session(&self, users: MutexGuard<'_, Vec<String>>) {
        let mut message = "No users in session.".into();
        if !users.is_empty() {
            message = format!("Users in session: {}", users.join(", "));
        }
        println!("{message}");
        self.discord.write_message(message.as_str()).await;
    }
}

#[cfg(test)]
mod tests {
    use serenity::async_trait;

    use crate::discord::DiscordAPI;

    use super::*;
    use std::sync::{Arc, Mutex};

    struct MemoryDiscord {
        messages: Arc<Mutex<Vec<String>>>,
    }

    #[async_trait]
    impl DiscordAPI for MemoryDiscord {
        async fn write_message(&self, message: &str) {
            self.messages.lock().unwrap().push(message.into());
        }
    }

    #[tokio::test]
    async fn test_joined() {
        assert_messages(vec![("JOINED", "User")], vec!["Users in session: User"]).await;
    }

    #[tokio::test]
    async fn test_duplicates_removed() {
        assert_messages(
            vec![("JOINED", "User"), ("JOINED", "User")],
            vec!["Users in session: User", "Users in session: User"],
        )
        .await;
    }

    #[tokio::test]
    async fn test_consequent_joined() {
        assert_messages(
            vec![("JOINED", "User"), ("JOINED", "Another")],
            vec!["Users in session: User", "Users in session: User, Another"],
        )
        .await;
    }

    #[tokio::test]
    async fn test_twojoin_oneleaving() {
        assert_messages(
            vec![("JOINED", "User"), ("JOINED", "Another"), ("LEFT", "User")],
            vec![
                "Users in session: User",
                "Users in session: User, Another",
                "Users in session: Another",
            ],
        )
        .await;
    }

    #[tokio::test]
    async fn test_no_user_left() {
        assert_messages(
            vec![("JOINED", "User"), ("LEFT", "User")],
            vec!["Users in session: User", "No users in session."],
        )
        .await;
    }

    async fn assert_messages(inputs: Vec<(&str, &str)>, outputs: Vec<&str>) {
        let inputs: Vec<String> = inputs
            .iter()
            .map(|(typ, msg)| {
                format!(
                    r#"
            {{
                "type": "{typ}",
                "userName": "{msg}"
            }}
            "#
                )
            })
            .collect();
        assert_payload(inputs, outputs).await;
    }

    async fn assert_payload(inputs: Vec<String>, outputs: Vec<&str>) {
        let messages = Arc::new(Mutex::new(Vec::new()));
        let mock = Arc::new(MemoryDiscord {
            messages: messages.clone(),
        });
        let lister = UserLister::new(mock);

        for msg in inputs {
            lister.json_message(msg.as_str()).await;
        }

        for (idx, msg) in outputs.iter().enumerate() {
            assert_eq!(&messages.lock().unwrap()[idx].as_str(), msg);
        }
    }
}
