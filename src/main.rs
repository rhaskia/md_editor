// mod buffer;
// mod cursor;
// mod document;
// use buffer::Buffer;
use clap::parser;
use clap::arg;
use clap::Command;

fn main() {
    let matches = Command::new("Text")
        .about("Yes")
        .arg(arg!([files] ... "Files to open"))
        .get_matches();

    let files = matches.get_many::<String>("files").unwrap().collect::<Vec<&String>>();
    println!("{:?}", files);

    // let args = load_arg_buffers();
    // println!("{args:?}");
}

// fn load_arg_buffers() -> Vec<Buffer> {
//     let args = std::env::args();
//     let _ = args.next();
//     let mut buffers = Vec::new();
//     let options = args.filter(|arg| arg.starts_with("--"))
//
//     if args.len() == 0 { 
//         let cwd = std::env::current_dir();
//         buffers.push(Buffer::dir(cwd));
//     }
//
//     for arg in args {
//         let path = std::path::load(arg);
//     }
//
//     buffers
// }
