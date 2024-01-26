pub mod pre {
    pub fn remove_comments_pass(text: &str) -> Option<String> {
        let first = text.split(';').next().unwrap_or("");

        if first.is_empty() {
            return None;
        }

        Some(String::from(first))
    }
}

pub mod post {}
