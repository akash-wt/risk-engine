use crate::engine::RiskEngine;
use futures_util::{SinkExt, StreamExt};
use std::sync::{Arc, Condvar, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub async fn feed(engine: Arc<(Mutex<RiskEngine>, Condvar)>) {
    let (lock, cond) = &*engine;

    let (socket, _) = connect_async("wss://hermes.pyth.network/ws").await.unwrap();
    let (mut write, mut read) = socket.split();

    write
        .send(Message::Text(
            r#"{
        "type": "subscribe",
        "ids": ["0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"]
    }"#
            .into(),
        ))
        .await
        .unwrap();
    while let Some(Ok(msg)) = read.next().await {
        if let Message::Text(text) = msg {
            let json: serde_json::Value = serde_json::from_str(&text).unwrap();
            if json["type"] != "price_update" {
                continue;
            }

            let raw_price: i64 = json["price_feed"]["price"]["price"]
                .as_str()
                .unwrap()
                .parse()
                .unwrap();

            let expo = json["price_feed"]["price"]["expo"].as_i64().unwrap();

            let mut mark_price = raw_price as u64;
            for _ in 0..(-expo - 4) {
                mark_price /= 10;
            }

            let mut eng = lock.lock().unwrap();
            eng.markets[0].market_price = mark_price;
            drop(eng);

            cond.notify_one();
            println!(
                "SOL mark_price: ${}.{:04}",
                mark_price / 10000,
                mark_price % 10000
            );
        }
    }
}
