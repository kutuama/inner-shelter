use warp::Filter;
use redis::AsyncCommands;
use futures_util::{StreamExt, SinkExt};
use tokio::sync::broadcast;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let redis_client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let (tx, _rx) = broadcast::channel(16);

    let filter = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let tx = tx.clone();
            let redis_client = redis_client.clone();
            ws.on_upgrade(move |socket| handle_socket(socket, tx, redis_client))
        });

    warp::serve(filter).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_socket(ws: warp::ws::WebSocket, tx: broadcast::Sender<String>, redis_client: redis::Client) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut redis_con = redis_client.get_async_connection().await.unwrap();
    let mut rx = tx.subscribe();

    let client_id = Uuid::new_v4().to_string(); // Generate a unique client ID

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
            tx.send(text.to_string()).unwrap();
        }
    }
}