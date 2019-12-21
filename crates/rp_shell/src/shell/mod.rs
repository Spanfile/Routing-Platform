mod commands;
mod completions;
mod history;
mod key_handlers;

use crate::error;
use anyhow::anyhow;
pub use commands::{Command, ExecutableCommand};
use completions::Completions;
use futures::stream::{self, StreamExt};
use history::HistoryEntry;
use key_handlers::KeyResult;
use rp_core::common::ShellMode;
use rp_log::*;
use std::{
    cell::RefCell,
    io::{self, Stdout, Write},
};
use termion::{
    self,
    cursor::DetectCursorPos,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Shell {
    pub running: bool,
    pub mode: ShellMode,
    pub prompt: String,
    stdout: RefCell<RawTerminal<Stdout>>,
    history: Vec<HistoryEntry>,
    history_index: Option<usize>,
    completions: Completions,
    buffer: Vec<char>,
    cursor_location: usize,
}

impl Shell {
    pub fn new() -> anyhow::Result<Self> {
        let stdout = io::stdout().into_raw_mode()?;
        stdout.suspend_raw_mode()?;
        Ok(Shell {
            running: true,
            mode: ShellMode::Operational,
            prompt: String::from(""),
            stdout: RefCell::new(stdout),
            history: Vec::new(),
            history_index: None,
            completions: Completions::new(Command::all_aliases()),
            buffer: Vec::new(),
            cursor_location: 0,
        })
    }

    pub async fn process_input(&mut self) -> anyhow::Result<Command> {
        match self.process_input_internal().await {
            Ok(command) => Ok(command),
            Err(e) => {
                trace!("Caught error while processing shell input, clearing buffer");
                self.buffer.clear();
                Err(e)
            }
        }
    }

    pub fn enter_mode(&mut self) -> anyhow::Result<()> {
        match self.mode {
            ShellMode::Operational => {
                self.mode = ShellMode::Configuration;
                Ok(())
            }
            ShellMode::Configuration => Err(error::ShellError::CannotEnterMode(self.mode).into()),
        }
    }

    pub fn exit_mode(&mut self) {
        match self.mode {
            ShellMode::Configuration => self.mode = ShellMode::Operational,
            ShellMode::Operational => self.running = false,
        }
    }

    pub fn print_history(&mut self) -> anyhow::Result<()> {
        for entry in &self.history {
            self.write(format_args!("{}\n", entry))?;
        }
        Ok(())
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Shell {
    fn write(&self, args: std::fmt::Arguments) -> anyhow::Result<()> {
        Ok(write!(self.stdout.try_borrow_mut()?, "{}", args)?)
    }

    fn flush(&self) -> anyhow::Result<()> {
        Ok(self.stdout.try_borrow_mut()?.lock().flush()?)
    }

    fn cursor_pos(&self) -> anyhow::Result<(u16, u16)> {
        Ok(self.stdout.try_borrow_mut()?.cursor_pos()?)
    }

    fn activate_raw_mode(&self) -> anyhow::Result<()> {
        Ok(self.stdout.try_borrow_mut()?.activate_raw_mode()?)
    }

    fn suspend_raw_mode(&self) -> anyhow::Result<()> {
        Ok(self.stdout.try_borrow_mut()?.suspend_raw_mode()?)
    }

    fn print_prompt(&mut self) -> anyhow::Result<()> {
        self.write(format_args!("{}", self.prompt.to_owned()))?;

        if !self.buffer.is_empty() {
            for b in self.buffer.iter() {
                self.write(format_args!("{}", b))?;
            }
        }

        self.flush()
    }

    async fn process_input_internal(&mut self) -> anyhow::Result<Command> {
        let input = loop {
            self.print_prompt()?;
            let input = self.read_input().await?;
            if input.trim().is_empty() {
                continue;
            }
            break input;
        };

        let input_args = input.to_owned();
        let args: Vec<&str> = input_args.split_whitespace().collect();

        if let Some(command_name) = args.first() {
            self.history.push(HistoryEntry::new(input));
            self.history_index = None;
            Ok(Command::new(
                command_name,
                args.iter().skip(1).map(|s| (*s).to_string()).collect(),
            )?)
        } else {
            Err(anyhow!("Split returned no args"))
        }
    }

    async fn read_input(&mut self) -> anyhow::Result<String> {
        let mut keys = stream::iter(io::stdin().keys());
        let mut reading = true;

        self.activate_raw_mode()?;

        while reading {
            if let Some(key) = keys.next().await {
                let key = key?;
                match key_handlers::get(key)?(key, self) {
                    Ok(result) => match result {
                        KeyResult::Stop => reading = false,
                        KeyResult::Skip => continue,
                        _ => (),
                    },
                    Err(e) => {
                        self.suspend_raw_mode()?;
                        return Err(e);
                    }
                }

                self.flush()?;
            }
        }

        self.suspend_raw_mode()?;

        let result = self.buffer.iter().collect();
        self.buffer.clear();
        self.cursor_location = 0;

        Ok(result)
    }
}
