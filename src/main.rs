use std::{net::TcpListener, sync::Arc, thread::spawn};

use discord::Discord;
use dotenv::dotenv;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
};
use user_lister::UserLister;

mod discord;
mod user_lister;

fn main() {
    dotenv().ok();
    let server = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let lister = UserLister {
                discord: Arc::new(Discord::new()),
            };
            let callback = |_: &Request, response: Response| Ok(response);
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();
            while let Ok(msg) = websocket.read_message() {
                if msg.is_text() {
                    lister.json_message(&msg.to_string());
                }
            }
        });
    }
}
