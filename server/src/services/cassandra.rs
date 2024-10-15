use scylla::{Session, SessionBuilder, QueryResult};
use scylla::transport::errors::{NewSessionError, QueryError};
use std::sync::Arc;

#[derive(Clone)]
pub struct CassandraSession {
    session: Arc<Session>,
}

impl CassandraSession {
    pub async fn new(node: &str) -> Result<Self, NewSessionError> {
        let session = SessionBuilder::new()
            .known_node(node)
            .build()
            .await?;
        Ok(Self { session: Arc::new(session) })
    }

    pub async fn query(&self, query: &str, values: &(impl scylla::frame::value::ValueList + Send + Sync)) -> Result<QueryResult, QueryError> {
        self.session.query(query, values).await
    }
}