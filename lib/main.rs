use log::debug;
use openai::{chat_completion, OpenAIMessage, OpenAIRequest};
use prompts::Prompts;
use serde_derive::{Deserialize, Serialize};
use std::env;
use strum_macros::{Display, EnumIter};

mod openai;
pub mod prompts;

#[derive(PartialEq, Eq, Hash, EnumIter, Display, Copy, Clone)]
/// Represents different actions that can be performed by the Iris assistant.
pub enum Actions {
    /// Explains a given context or input.
    Explain,
    /// Summarizes a given context or input.
    Summarize,
    /// Edits a given context or input.
    Edit,
    /// Asks a question based on the given context or input.
    Ask,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IrisConfig {
    /// Name of the environment variable where the API key is saved
    openai_api_key: String,

    /// URL of the Open AI API (including '/v1')
    openai_api_endpoint: String,

    /// Prompt templates for the different actions
    prompts: Prompts,
}

impl Default for IrisConfig {
    fn default() -> Self {
        Self {
            openai_api_key: "OPENAI_API_KEY".to_owned(),
            openai_api_endpoint: "https://api.openai.com/v1".to_owned(),
            prompts: Prompts::default(),
        }
    }
}

impl IrisConfig {
    /// Load Iris configuration including environment variables
    pub fn load() -> Result<Self, String> {
        debug!("Load Config");

        let mut conf: IrisConfig = match confy::load("iris", "config") {
            Ok(config) => config,
            Err(e) => return Err(format!("Unable to load Iris config: {}", e)),
        };

        let var_name = conf.openai_api_key.clone();
        conf.openai_api_key = match env::var(var_name) {
            Ok(key) => key,
            Err(e) => return Err(format!("Error retrieving {}: {}", conf.openai_api_key, e)),
        };

        Ok(conf)
    }
}

/// Executes the Iris assistant.
///
/// # Arguments
///
/// * `action` - A reference to an `Actions` enum variant indicating the desired action (e.g., Explain, Summarize).
/// * `context` - A string slice providing additional context for the action.
/// * `user_input` - An optional string slice containing user input for the action.
/// * `config` - The `IrisConfig` struct containing necessary configuration details.
///
/// # Returns
///
/// An `Option<String>` where the value is `Some(String)` if a response is successfully received from the LLM,
/// and `None` if an error occurs during the process.
pub fn run(
    action: &Actions,
    context: &str,
    user_input: Option<&str>,
    config: IrisConfig,
) -> Option<String> {
    debug!("Run Action");

    let prompt = match action {
        Actions::Explain => &config.prompts.explain,
        Actions::Summarize => &config.prompts.summarize,
        Actions::Edit => &config.prompts.edit,
        Actions::Ask => &config.prompts.ask,
    };

    let openai_req = OpenAIRequest {
        model: "default".to_owned(),
        messages: vec![OpenAIMessage {
            content: prompt.format(context, user_input),
            role: "user".to_owned(),
        }],
        stream: false,
    };

    let response = chat_completion(
        &openai_req,
        &config.openai_api_key,
        &config.openai_api_endpoint,
    );

    Some(response.choices[0].message.content.clone())
}
