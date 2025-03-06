use serde::{Deserialize, Serialize};

use crate::model::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub model: Model,
    pub messages: Vec<Message>,
}
