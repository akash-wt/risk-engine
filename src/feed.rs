use std::sync::Arc;

use tokio::sync::{Mutex, Notify};
use tokio_tungstenite::connect_async;

use crate::engine::RiskEngine;

pub async fn feed(engine: Arc<(std::sync::Mutex<RiskEngine>, Notify)>) {
    let (socket, _) = connect_async("wss//hermes.pyth.network/ws").await.unwrap();
    let (mut write, mut read) = socket.split();


}
