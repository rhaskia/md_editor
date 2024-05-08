use crate::buffer::Buffer;

pub struct App {
    buffers: Vec<Buffer>
}

impl App {
    pub fn new() -> Self {
        Self { buffers: Vec::new() }
    }

    pub fn load_buffers(&mut self, paths: Vec<&String>) {
        if paths.is_empty() { 
            //CWD
        }

    }
}
