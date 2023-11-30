use std::fs;

pub fn read_string(path: &str) -> Vec<String> {
    match fs::read_to_string(path) {
        Ok(content) => content.lines().map(String::from).collect(),
        Err(_) => Vec::new(),
    }
}
