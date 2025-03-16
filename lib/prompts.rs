pub struct Prompt {
    content: &'static str,
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

// pub struct ActionPrompt {
// system: Prompt,
// user: Prompt,
// }

pub static EXPLAIN_PROMPT: Prompt = Prompt {
    content: "---TEXT---\n{context}\n---END OF TEXT---\n\nPlease provide a short and precise explanation of the text given above."
};

pub static SUMMARIZE_PROMPT: Prompt = Prompt {
    content: "---TEXT---\n{context}\n---END OF TEXT---\n\nPlease provide a short and precise summary of the text given above."
};

pub static EDIT_PROMPT: Prompt = Prompt {
    content: "---TEXT---\n{context}\n---END OF TEXT---\n\nPlease edit the text above to fulfill the following request: {user_input}"
};

pub static ASK_PROMPT: Prompt = Prompt {
    content: "---TEXT---\n{context}\n---END OF TEXT---\n\nBased on the text above answer the following question: {user_input}\nOnly answer based on the information given in the Text."
};
