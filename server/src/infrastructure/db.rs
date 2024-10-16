use scylla::transport::session::{Session, SessionConfig};
use std::sync::Arc;
use crate::errors::AppError;

pub async fn get_db_session(cassandra_uri: &str) -> Result<Arc<Session>, AppError> {
    let mut session_config = SessionConfig::new();
    session_config.add_known_node(cassandra_uri);
    
    // Attempt to connect to the Cassandra session
    let session = Session::connect(session_config).await.map_err(|e| AppError::DbError(e.to_string()))?;
    
    Ok(Arc::new(session))
}
