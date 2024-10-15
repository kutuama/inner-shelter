use redis::Client;

pub fn get_redis_client() -> Client {
    redis::Client::open("redis://127.0.0.1/").unwrap()
}
