
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BrainFile {
    pub compile: Vec<String>,
}

pub fn is_format_valid(content: String) -> bool {
    let result: Result<BrainFile, serde_json::Error> = serde_json::from_str(&content);

    if let Ok(_brain_file) = result {
        return true;
    }

    false
}


pub fn parse(content: String) -> BrainFile {
    serde_json::from_str(&content).unwrap()
}
