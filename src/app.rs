use crate::buffer::Buffer;
use crate::renderer::Renderer;
use crossterm::event::Event;
use crossterm::event::read;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io::{Error, stdout, Stdout, Write};
use modalkit::env::vim::keybindings::{default_vim_keys, VimMachine};
use modalkit::key::TerminalKey;
use modalkit::keybindings::BindingMachine;
use fehler::throws;
use modalkit::actions::*;
use modalkit::editing::context::EditContext;
use modalkit::prelude::{CommandType, Specifier, Char};

pub struct App {
    buffers: Vec<Buffer>,
    state: VimMachine<TerminalKey>,
    command_type: Option<CommandType>,
    bar_input: String,
    renderer: Renderer,
}

impl App {
    pub fn new() -> Self {
        Self { 
            buffers: Vec::new(), 
            state: default_vim_keys(),
            command_type: None,
            bar_input: String::new(),
            renderer: Renderer::new(),
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

    #[throws]
    pub fn run(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        self.renderer.enter_screen();
        self.update();
        disable_raw_mode()?;
        self.renderer.exit_screen();
        Ok(())
    }

    #[throws]
    pub fn update(&mut self) {
        loop {
            let event = read()?;

            use Event::*;
            match event {
                Key(key) => self.state.input_key(key.into()), 
                _ => {}
            }

            while let Some((act, ctx)) = self.state.pop() {
                //println!("\r{act:?}");
                self.handle_action(act, ctx);
            }

            if self.buffers().len() == 0 {
                break;
            }
        }
    }

    pub fn handle_action(&mut self, act: Action, ctx: EditContext) {
        use Action::*;
        match act {
            Editor(editor) => self.handle_editor(editor, ctx),
            Command(command) => self.handle_command(command, ctx),
            CommandBar(cb) => self.handle_c_bar(cb, ctx),
            Prompt(prompt) => self.handle_prompt(prompt, ctx),
            NoOp => {},
            _ => {}
        }
    } 

    pub fn handle_prompt(&mut self, action: PromptAction, ctx: EditContext) {
        use PromptAction::*;
        match action {
            Submit => self.submit(),
            _ => {}
        }
    }

    pub fn submit(&mut self) {
        if let Some(bar) = self.command_type {
            use CommandType::*;
            match bar {
                Command => self.call_command(),
                Search => self.search(),
            }
        }

        // New line probs
    }

    pub fn search(&mut self) {}

    pub fn call_command(&mut self) {
        match self.bar_input.as_str() {
            "q" => self.close_buffer(),
            _ => self.renderer.draw_command_out(
                &format!("command not found: {}", self.bar_input)),
        }
    }

    pub fn close_buffer(&mut self) { self.buffers.pop(); }

    pub fn handle_editor(&mut self, action: EditorAction, ctx: EditContext) {
        use EditorAction::*;
        match action {
            InsertText(text) => self.handle_insert(text),
            _ => {}
        }
    }

    pub fn handle_insert(&mut self, text: InsertTextAction) {
        use InsertTextAction::*;
        use Specifier::Exact;
        match text {
            Type(Exact(c), ..) => self.insert_char(c),
            _ => {}
        }
    }

    pub fn insert_char(&mut self, c: Char) {
        use Char::*;
        let text = match c {
            Single(c) => String::from_iter(&[c]),
            Digraph(a, b) => String::from_iter(&[a, b]),
            _ => String::new(),
        };

        if let Some(bar) = &self.command_type {
            self.bar_input.push_str(&text);
            self.renderer.draw_command_bar(&self.bar_input, bar);
        }
        // Insert into doc
    }

    pub fn handle_command(&mut self, action: CommandAction, ctx: EditContext) {
        println!("{:?}", action);
        match action {
            _ => {}
        }
    }

    pub fn handle_c_bar(&mut self, action: CommandBarAction, ctx: EditContext) {
        use CommandBarAction::*;
        match action {
            Focus(command_type) => {
                self.command_type = Some(command_type);
                self.bar_input = String::new();
            },
            Unfocus => self.command_type = None,    
        } 
        self.renderer.draw_command_out("");
    }
}
