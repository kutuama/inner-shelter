use redis::{AsyncCommands, Client, aio::{Connection, PubSub}, RedisError, Msg};
use crate::config::Config;
use crate::errors::AppError;
use crate::domain::entities::PositionUpdate;
use crate::domain::game_repository::GameRepository;
use async_trait::async_trait;
use futures_util::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct RedisGameRepository {
    client: Arc<Client>,
}

impl RedisGameRepository {
    pub fn new(config: &Config) -> Result<Self, AppError> {
        let client = Arc::new(Client::open(config.redis_url.clone()).map_err(|e| {
            AppError::DbError(format!("Failed to create Redis client: {}", e))
        })?);
        Ok(Self { client })
    }

    async fn get_connection(&self) -> Result<Connection, AppError> {
        self.client.get_async_connection().await.map_err(|e| {
            AppError::DbError(format!("Failed to connect to Redis: {}", e))
        })
    }

    async fn get_pubsub_connection(&self) -> Result<PubSub, AppError> {
        let conn = self.client.get_async_connection().await.map_err(|e| {
            AppError::DbError(format!("Failed to connect to Redis: {}", e))
        })?;
        Ok(conn.into_pubsub())
    }
}

pub struct RedisStream {
    pubsub: PubSub,
}

impl RedisStream {
    pub fn new(pubsub: PubSub) -> Self {
        Self { pubsub }
    }
}

impl Stream for RedisStream {
    type Item = Result<Msg, RedisError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Get a mutable reference to `pubsub`
        let pubsub = &mut self.pubsub;

        // Get the message stream from `pubsub`
        let on_message = pubsub.on_message();
        futures_util::pin_mut!(on_message);

        // Poll the next message
        match on_message.poll_next_unpin(cx) {
            Poll::Ready(Some(msg)) => Poll::Ready(Some(Ok(msg))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[async_trait]
impl GameRepository for RedisGameRepository {
    async fn get_position(&self, username: &str) -> Result<(i32, i32), AppError> {
        let mut conn = self.get_connection().await?;
        let key = format!("player:{}", username);
        let exists: bool = conn.exists(&key).await.map_err(|e| {
            AppError::DbError(format!("Failed to check if key exists: {}", e))
        })?;

        if exists {
            let (x, y): (i32, i32) = conn.hget(&key, &["x", "y"]).await.map_err(|e| {
                AppError::DbError(format!("Failed to get position from Redis: {}", e))
            })?;
            Ok((x, y))
        } else {
            Ok((1, 1))
        }
    }

    async fn set_position(&self, username: &str, x: i32, y: i32) -> Result<(), AppError> {
        let mut conn = self.get_connection().await?;
        let key = format!("player:{}", username);
        let _: () = conn.hset_multiple(&key, &[("x", x), ("y", y)]).await.map_err(|e| {
            AppError::DbError(format!("Failed to set position in Redis: {}", e))
        })?;
        Ok(())
    }

    async fn publish_position_update(&self, update: &PositionUpdate) -> Result<(), AppError> {
        let mut conn = self.get_connection().await?;
        let message = serde_json::to_string(update).unwrap();
        let _: () = conn.publish("game_channel", message).await.map_err(|e| {
            AppError::DbError(format!("Failed to publish to Redis: {}", e))
        })?;
        Ok(())
    }

    async fn subscribe(
        &self,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<Msg, RedisError>> + Send>>, AppError> {
        let mut pubsub = self.get_pubsub_connection().await?;
        pubsub.subscribe("game_channel").await.map_err(|e| {
            AppError::DbError(format!("Failed to subscribe to Redis channel: {}", e))
        })?;

        // Create a RedisStream that owns the pubsub
        let stream = RedisStream::new(pubsub);

        // Return the stream, ensuring the pubsub lives as long as the stream
        Ok(Box::pin(stream))
    }
}
