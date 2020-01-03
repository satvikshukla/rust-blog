use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

impl LoginRequest {
    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }
}
