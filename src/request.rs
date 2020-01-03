use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreatePostRequest {
    body: String,
}

impl CreatePostRequest {
    pub fn get_body(&self) -> &str {
        &self.body
    }
}

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
