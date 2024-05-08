use crate::buffer::Buffer;

struct App {
    buffers: Vec<Buffer>
}

impl App {
    pub fn new() -> Self {
        Self { buffers: Vec::new() }
    }
}
