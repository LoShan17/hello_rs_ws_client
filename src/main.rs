use tungstenite::{connect, Message};
use url::Url;
use serde_json;
fn main() {
    // Connect to the WS server locally
    let (mut socket, _response) = connect(Url::parse("ws://localhost:8765").unwrap()).expect("Can't connect");
    // Write a message containing "Hello, Test!" to the server
    socket.write_message(Message::Text("Hello, Test!".into())).unwrap();
    
    // Loop forever, handling parsing each message
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            tungstenite::Message::Ping(_v) => { "{\"error\": null, \"result\": \"Ping\"}".to_string() }
            tungstenite::Message::Pong(_v) => { "{\"error\": null, \"result\": \"Pong\"}".to_string() }
            _ => { panic!() }
        };
        let parsed: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");
        println!("{:?}", parsed["result"]);
    }
}