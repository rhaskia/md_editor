use crate::cursor::Cursor;
use crate::document::Document;
use std::path::Path;

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

#[derive(Debug)]
pub struct Directory { path: Path, cursor: Cursor }

#[derive(Debug)]
pub struct File { path: Path, cursor: Cursor, buffer: Document }

impl Directory { 
    pub fn new(path: Path) -> Self {
        Self { path, cursor: Cursor::default() }
    }
}
