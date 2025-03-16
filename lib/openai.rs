use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
    pub stream: bool,
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIResponse {
    pub id: String,
    pub object: String,
    pub created: i32,
    pub model: String,
    pub choices: Vec<OpenAIChoices>,
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIChoices {
    pub message: OpenAIMessage,
}

pub fn chat_completion(req: &OpenAIRequest, api_key: &str, api_base: &str) -> OpenAIResponse {
    debug!("Execute Chat Completions to: {}", api_base);

    let client = reqwest::blocking::Client::new();

    let res = client
        .post(format!("{api_base}/chat/completions"))
        .bearer_auth(api_key)
        .json(req)
        .send()
        .expect("OpenAI API request failed!");

    res.json()
        .expect("Deserialization of OpenAI response failed!")
}
