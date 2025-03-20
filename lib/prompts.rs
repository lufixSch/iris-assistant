use serde_derive::{Deserialize, Serialize};

/// Represents a prompt with content that can be formatted.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    /// The content of the prompt, which may include placeholders like `{context}` and `{user_input}`.
    pub content: String,
}

impl Prompt {
    /// Formats the prompt's content by replacing placeholders with actual values.
    ///
    /// # Arguments
    ///
    /// - `context` - A string slice that holds the context to replace `{context}` placeholder.
    /// - `user_input` - An optional string slice that holds the user input to replace `{user_input}` placeholder.
    ///
    /// # Returns
    ///
    /// A new string with placeholders replaced by actual values.
    pub fn format(&self, context: &str, user_input: Option<&str>) -> String {
        let tmp = self.content.replace("{context}", context);

        match user_input {
            Some(val) => tmp.replace("{user_input}", val),
            _ => tmp,
        }
    }
}

/// Represents a collection of different types of prompts.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompts {
    /// The prompt used for explaining content.
    pub explain: Prompt,
    /// The prompt used for summarizing content.
    pub summarize: Prompt,
    /// The prompt used for editing content.
    pub edit: Prompt,
    /// The prompt used for asking questions.
    pub ask: Prompt,
}

impl Default for Prompts {
    /// Provides default prompts with predefined content.
    fn default() -> Self {
        Self {
            explain: Prompt { content: "---TEXT---\n{context}\n---END OF TEXT---\n\nPlease provide a short and precise explanation of the text given above.".to_owned() },
            summarize: Prompt { content: "---TEXT---\n{context}\n---END OF TEXT---\n\nPlease provide a short and precise summary of the text given above.".to_owned() },
            edit: Prompt { content: "---TEXT---\n{context}\n---END OF TEXT---\n\nPlease edit the text above to fulfill the following request: {user_input}".to_owned() },
            ask: Prompt { content: "---TEXT---\n{context}\n---END OF TEXT---\n\nBased on the text above answer the following question: {user_input}\nOnly answer based on the information given in the Text.".to_owned() }
        }
    }
}

