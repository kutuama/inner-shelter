use redis::AsyncCommands;
use redis::aio::Connection;

#[allow(dead_code)]
pub async fn set_key(con: &mut Connection, key: &str, value: &str) {
    let _: () = con.set(key, value).await.unwrap();
}
