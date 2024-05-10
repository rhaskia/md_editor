use std::io::{Error, stdout, Stdout, Write};
use modalkit::prelude::CommandType;
use crossterm::execute;
use crossterm::cursor::MoveTo;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType};
use crate::buffer::{Directory, File, Buffer};

pub struct Renderer {
    out: Stdout,
    cols: u16, 
    rows: u16,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            out: stdout(),
            rows: 24,
            cols: 80,
        }
    }

    pub fn draw_command_bar(&mut self, input: &str, bar_type: &CommandType) {
        use CommandType::*;
        let pre = match bar_type {
            Search => "search=",
            Command => ":",
        };
        self.draw_command_out(&format!("{pre}{input}"))
    }

    pub fn draw_command_out(&mut self, input: &str) {
        execute!(&self.out, MoveTo(0, self.rows));
        execute!(&self.out, Clear(ClearType::CurrentLine));
        write!(&self.out, "{}", input);
        self.out.flush();
    }

    pub fn enter_screen(&mut self) {
        execute!(&self.out, EnterAlternateScreen);
        execute!(&self.out, Clear(ClearType::All));
        self.out.flush();
    }

    pub fn exit_screen(&mut self) {
        execute!(&self.out, LeaveAlternateScreen);
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.rows = rows;
        self.cols = cols;
    }
}

impl Renderer {
    pub fn draw_buffer(&mut self, buffer: &Buffer) {
        execute!(&self.out, MoveTo(0, 1));
        use Buffer::*;
        match buffer {
            Directory(d) => self.draw_dir(d),
            File(f) => self.draw_file(f),
        }
    } 

    pub fn draw_dir(&mut self, dir: &Directory) {
        write!(&self.out, "{:?}", dir.path); 
    }

    pub fn draw_file(&mut self, file: &File) {
        write!(&self.out, "{}", file.document.text()); 
    }
}
