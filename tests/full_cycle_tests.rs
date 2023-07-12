use tungstenite::{connect, Message};
use url::Url;

#[test]
#[ignore]
fn user_joined() {
    send(
        r#"
    {
        "type": "JOINED",
        "userName": "User"
    }
    "#,
    );
}

fn send(json: &str) {
    // let mut cmd = std::process::Command::new("cargo")
    //     .arg("run")
    //     .spawn()
    //     .unwrap();

    let (mut socket, _) =
        connect(Url::parse("ws://localhost:3000/socket").unwrap()).expect("Can't connect");
    socket.write_message(Message::Text(json.into())).unwrap();

    //cmd.kill().unwrap();
}
