use crate::cursor::Cursor;

#[derive(Debug)]
pub enum Buffer {
    Directory(Directory),
    File(File),
}

impl Buffer {
    pub fn dir(path: Path) -> Buffer {
        Self::Directory(Directory::new(path))
    }
}

pub struct Directory { path: Path, cursor: Cursor }
pub struct File { path: Path, cursor: Cursor, buffer: String } // Rope

impl Directory { 
    pub fn new(path: Path) -> Self {
        Self { path, cursor: Cursor::default() }
    }
}
