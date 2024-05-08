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
            self.buffers.push(
                Buffer::dir(
                    std::env::current_dir().expect("CWD not found")));
        }

        for path in paths {
            self.buffers.push(Buffer::load(path));
        }
    }

    pub fn buffers(&self) -> &Vec<Buffer> {
        &self.buffers
    }
}
