mod commands;
mod completions;

pub use commands::{Command, ExecutableCommand};

#[derive(Debug)]
pub struct Shell {
    pub running: bool,
    mode: ShellMode,
}

#[derive(Debug)]
pub enum ShellMode {
    Operational,
    Configuration,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            running: true,
            mode: ShellMode::Operational,
        }
    }

    pub fn process_input(&mut self) -> Command {
        unimplemented!();
    }

    pub fn enter_mode(&mut self) {
        match self.mode {
            ShellMode::Operational => self.mode = ShellMode::Configuration,
            ShellMode::Configuration => (), // TODO: return error or something?
        }
    }

    pub fn exit_mode(&mut self) {
        match self.mode {
            ShellMode::Configuration => self.mode = ShellMode::Operational,
            ShellMode::Operational => self.running = false,
        }
    }
}
