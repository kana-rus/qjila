pub fn snake_cased(s: &str) -> String {
    let mut chars = Vec::new();

    for c in s.chars() {
        if !chars.is_empty() && c.is_ascii_uppercase() {
            chars.push('_')
        }
        chars.push(c.to_ascii_lowercase())
    }

    chars.into_iter().collect()
}
