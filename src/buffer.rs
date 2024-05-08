use crate::cursor::Cursor;
use crate::document::Document;
use std::path::PathBuf;
use std::fs;

#[derive(Debug)]
pub enum Buffer {
    Directory(Directory),
    File(File),
}

impl Buffer {
    pub fn dir(path: PathBuf) -> Buffer {
        Self::Directory(Directory::new(path))
    }

    pub fn file(path: PathBuf) -> Buffer {
        Self::File(File::new(path))
    }

    pub fn load_from_dir(dir: &str) -> Buffer {
        let path = PathBuf::from(dir);
        match path.is_dir() {
            true => Self::dir(path),
            false => Self::file(path),
        }
    }
}

#[derive(Debug)]
pub struct Directory { path: PathBuf, cursor: Cursor }

#[derive(Debug)]
pub struct File { path: PathBuf, cursor: Cursor, document: Document }

impl Directory { 
    pub fn new(path: PathBuf) -> Self {
        Self { path, cursor: Cursor::default() }
    }
}

impl File {
    pub fn new(path: PathBuf) -> Self {
        Self { path: path.clone(), cursor: Cursor::default(), document: Document::load(path) }
    }
}
