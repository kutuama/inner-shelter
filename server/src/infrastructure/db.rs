use scylla::transport::session::{Session, SessionConfig};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn get_db_session(cassandra_uri: &str) -> Arc<Mutex<Session>> {
    let mut session_config = SessionConfig::new();
    session_config.add_known_node(cassandra_uri);
    let session = Session::connect(session_config).await.expect("Cassandra connection failed");
    Arc::new(Mutex::new(session))
}
