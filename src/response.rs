use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    message: String,
}

impl LoginResponse {
    pub fn new(message: String) -> LoginResponse {
        LoginResponse {message: message}
    }
    pub fn get_message(&self) -> &str {
        &self.message
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateResponse {
    message: String,
}

impl CreateResponse {
    pub fn new(message: String) -> CreateResponse {
        CreateResponse { message: message }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}
