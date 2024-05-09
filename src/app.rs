use crate::buffer::Buffer;
use crossterm::event::Event;
use crossterm::event::read;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io::{Error, stdout, Stdout};
use modalkit::env::vim::keybindings::{default_vim_keys, VimMachine};
use modalkit::key::TerminalKey;
use modalkit::keybindings::BindingMachine;

pub struct App {
    stdout: Stdout,
    buffers: Vec<Buffer>,
    state: VimMachine<TerminalKey>
}

impl App {
    pub fn new() -> Self {
        Self { 
            buffers: Vec::new(), 
            stdout: stdout(), 
            state: default_vim_keys() 
        }
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

    pub fn run(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        self.update();
        disable_raw_mode()?;
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), Error> {
        loop {
            let event = read()?;

            use Event::*;
            match event {
                Key(key) => self.state.input_key(key.into()), 
                _ => {}
            }
            let (act, ctx) = match self.state.pop() { Some(s) => s, None => continue };
            println!("{act:?}, {ctx:?}");
        }
    }
}
