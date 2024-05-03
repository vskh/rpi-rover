use gloo_net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};

fn test() {
    let mut ws = WebSocket::open("wss://echo.websocket.org").unwrap();
    let (mut write, mut read) = ws.split();

    spawn_local(async move {
        write.send(Message::Text(String::from("test"))).await.unwrap();
        write.send(Message::Text(String::from("test 2"))).await.unwrap();
    });

    spawn_local(async move {
        while let Some(msg) = read.next().await {
            // console_log!(format!("1. {:?}", msg))
        }
        // console_log!("WebSocket Closed")
    });
}