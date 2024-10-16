#[derive(Clone)]
pub struct User {
    pub username: String,
    pub token: Option<String>,
}

impl User {
    pub fn new(username: String) -> Self {
        Self {
            username,
            token: None,
        }
    }
}
