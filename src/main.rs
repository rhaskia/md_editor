mod buffer;
mod cursor;
use buffer::Buffer;

fn main() {
    let args = load_arg_buffers();
    println!("{args:?}");
}

fn load_arg_buffers() -> Vec<Buffer> {
    let args = std::env::args();
    let _ = args.next();
    let mut buffers = Vec::new();

    if args.len() == 0 { buffers.push( Buffer::dir(std::path::cwd()) ) }

    for arg in args {
        let path = std::path::load(arg);
    }
}
