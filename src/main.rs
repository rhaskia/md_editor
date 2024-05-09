mod buffer;
mod app;
mod cursor;
mod document;

use buffer::Buffer;
use clap::{Command, arg};
use clap::parser::ValuesRef;
use app::App;
use std::io;

fn main() -> Result<(), std::io::Error> {
    let mut stdout = io::stdout();

    let matches = Command::new("Text")
        .about("Yes")
        .arg(arg!([files] ... "Files to open"))
        .get_matches();

    let files = matches.get_many::<String>("files")
        .unwrap_or_default()
        .collect::<Vec<&String>>();
    
    let mut app = App::new();
    app.load_buffers(files);
    println!("{:?}", app.buffers());

    app.run();

    Ok(())
}

