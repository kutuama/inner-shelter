use warp::ws::WebSocket;
use tokio::sync::broadcast;
use futures_util::{StreamExt, SinkExt};
use redis::AsyncCommands;
use redis::aio::Connection;
use uuid::Uuid;

pub async fn handle_socket(ws: WebSocket, tx: broadcast::Sender<String>, redis_client: redis::Client, user_id: String) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut redis_con: Connection = redis_client.get_async_connection().await.unwrap();
    let mut rx = tx.subscribe();

    let client_id = Uuid::new_v4().to_string();

    // Broadcast incoming text to all clients
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if ws_tx.send(warp::ws::Message::text(msg)).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(msg)) = ws_rx.next().await {
        if msg.is_text() {
            let text = msg.to_str().unwrap();
            let _: () = redis_con.set(&client_id, text).await.unwrap();

            // Broadcast the message to all other clients
            tx.send(format!("{}: {}", user_id, text)).unwrap();
        }
    }
}
