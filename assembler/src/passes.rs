pub mod pre {
    pub fn remove_comments_pass(text: &str) -> String {
        text.lines()
            .filter_map(|l| {
                if l.is_empty() || l.starts_with(';') {
                    None
                } else {
                    Some(l.split(';').next().unwrap_or("").to_string())
                }
            })
            .collect()
    }
}

pub mod post {}
