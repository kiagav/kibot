use std::{net::TcpListener, sync::Arc};

use discord::Discord;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};
use user_lister::UserLister;

mod discord;
mod user_lister;

#[tokio::main]
async fn main() {
    let lister = Arc::new(UserLister::new(Arc::new(Discord::new())));
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in server.incoming() {
        let callback = |_: &Request, response: Response| Ok(response);
        let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();
        while let Ok(msg) = websocket.read() {
            if msg.is_text() {
                lister.json_message(&msg.to_string()).await;
            }
        }
    }
}
