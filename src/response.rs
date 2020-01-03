use crate::queries::Post;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreatePostResponse {
    message: String,
}

impl CreatePostResponse {
    pub fn new(message: String) -> CreatePostResponse {
        CreatePostResponse { message: message }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    message: String,
}

impl CreateUserResponse {
    pub fn new(message: String) -> CreateUserResponse {
        CreateUserResponse { message: message }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}

// #[derive(Serialize, Deserialize)]
// pub struct Post {
//     body: String,
// }

// impl Post {
//     pub fn new(body: String) -> Post {
//         Post { body: body }
//     }
// }

#[derive(Serialize, Deserialize)]
pub struct GetPostResponse {
    posts: Vec<Post>,
}

impl GetPostResponse {
    pub fn new(posts: Vec<Post>) -> GetPostResponse {
        GetPostResponse { posts: posts }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    message: String,
}

impl LoginResponse {
    pub fn new(message: String) -> LoginResponse {
        LoginResponse { message: message }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}
