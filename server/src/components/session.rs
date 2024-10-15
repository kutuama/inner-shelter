#[allow(dead_code)]
pub struct SessionComponent {
    pub user_id: String,
    pub session_start: chrono::DateTime<chrono::Utc>,
}
