pub fn remove_comments_pass(text: &mut Vec<String>) {
    *text = text
        .iter()
        .filter(|l| !l.is_empty())
        .filter(|l| !l.starts_with(';'))
        .map(|l| l.split(';').next().unwrap_or("").to_string())
        .collect();
}
