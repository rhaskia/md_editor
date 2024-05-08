use std::path::PathBuf;

#[derive(Debug)]
pub struct Document {
    buffer: String
}

impl Document {
    pub fn load(path: PathBuf) -> Document {
        let buffer = std::fs::read_to_string(path).unwrap();
        Document { buffer }
    }
}
