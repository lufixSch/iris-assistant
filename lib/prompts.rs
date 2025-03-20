use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub content: String,
}

impl Prompt {
    pub fn format(&self, context: &str, user_input: Option<&str>) -> String {
        let tmp = self.content.replace("{context}", context);

        match user_input {
            Some(val) => tmp.replace("{user_input}", val),
            _ => tmp,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompts {
    pub explain: Prompt,
    pub summarize: Prompt,
    pub edit: Prompt,
    pub ask: Prompt,
}

impl Default for Prompts {
    fn default() -> Self {
        Self {
            explain: Prompt { content: "---TEXT---\n{context}\n---END OF TEXT---\n\nPlease provide a short and precise explanation of the text given above.".to_owned() },
            summarize: Prompt { content: "---TEXT---\n{context}\n---END OF TEXT---\n\nPlease provide a short and precise explanation of the text given above.".to_owned() },
            edit: Prompt { content: "---TEXT---\n{context}\n---END OF TEXT---\n\nPlease provide a short and precise explanation of the text given above.".to_owned() },
            ask: Prompt { content: "---TEXT---\n{context}\n---END OF TEXT---\n\nPlease provide a short and precise explanation of the text given above.".to_owned() }
        }
    }
}
