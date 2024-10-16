use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

impl LoginData {
    pub fn validate(&self) -> Result<(), String> {
        if self.username.trim().is_empty() {
            return Err("Username cannot be empty".into());
        }
        if self.password.is_empty() {
            return Err("Password cannot be empty".into());
        }
        Ok(())
    }
}


#[derive(Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
}

impl RegisterData {
    pub fn validate(&self) -> Result<(), String> {
        if self.username.trim().is_empty() {
            return Err("Username cannot be empty".into());
        }
        if self.username.len() < 3 {
            return Err("Username must be at least 3 characters long".into());
        }
        Ok(())
    }
}
