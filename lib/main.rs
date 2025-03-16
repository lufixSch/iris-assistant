use log::{debug, error, info};
use openai::{chat_completion, OpenAIMessage, OpenAIRequest};
use prompts::{ASK_PROMPT, EDIT_PROMPT, EXPLAIN_PROMPT, SUMMARIZE_PROMPT};
use std::env;
use strum_macros::{Display, EnumIter};

mod openai;
pub mod prompts;

#[derive(PartialEq, EnumIter, Display, Copy, Clone)]
pub enum Actions {
    Explain,
    Summarize,
    Edit,
    Ask,
}

pub fn run(action: &Actions, context: &str, user_input: Option<&str>) -> Option<String> {
    debug!("Run Action");

    let api_key = env::var("OPEN_WEBUI_API_KEY").unwrap().to_string();

    let prompt = match action {
        Actions::Explain => &EXPLAIN_PROMPT,
        Actions::Summarize => &SUMMARIZE_PROMPT,
        Actions::Edit => &EDIT_PROMPT,
        Actions::Ask => &ASK_PROMPT,
    };

    let openai_req = OpenAIRequest {
        model: "default".to_owned(),
        messages: vec![OpenAIMessage {
            content: prompt.format(context, user_input),
            role: "user".to_owned(),
        }],
        stream: false,
    };

    let response = chat_completion(&openai_req, &api_key, "http://ltd-nas.lan:1132/api");

    Some(response.choices[0].message.content.clone())
}
