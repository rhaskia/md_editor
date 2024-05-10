use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug)]
pub struct Document {
    buffer: String,
    last_saved: Instant, // for changes that may not be from this app
}

impl Document {
    pub fn load(path: PathBuf) -> Document {
        let buffer = std::fs::read_to_string(path).unwrap();
        Document { buffer, last_saved: Instant::now() }
    }

    pub fn text(&self) -> &str {
        &self.buffer
    }
}
