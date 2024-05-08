mod buffer;
mod app;
mod cursor;
mod document;
use buffer::Buffer;
use clap::arg;
use clap::Command;
use clap::parser::ValuesRef;
use app::App;

fn main() {
    let matches = Command::new("Text")
        .about("Yes")
        .arg(arg!([files] ... "Files to open"))
        .get_matches();

    let files = matches.get_many::<String>("files")
        .unwrap_or_default()
        .collect::<Vec<&String>>();

    println!("{:?}", files);

    let mut app = App::new();
    app.load_buffers(files);

    // let args = load_arg_buffers();
    // println!("{args:?}");
}
