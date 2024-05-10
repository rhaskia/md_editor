use std::io::{Error, stdout, Stdout, Write};
use modalkit::prelude::CommandType;
use crossterm::execute;
use crossterm::cursor::MoveTo;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType};

pub struct Renderer {
    out: Stdout,
    cols: usize, 
    rows: usize,
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
            Search => "/",
            Command => ":",
        };
        self.draw_command_out(&format!("{pre}{input}"))
    }

    pub fn draw_command_out(&mut self, input: &str) {
        execute!(&self.out, MoveTo(0, self.rows as u16));
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
}
