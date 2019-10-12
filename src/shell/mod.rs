mod commands;
mod completions;
mod shell_error;

use crate::error;
pub use commands::{Command, CommandError, ExecutableCommand};
pub use shell_error::ShellError;
use std::io::{self, Stdout, Write};
use termion::{
    self, cursor,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Shell {
    pub running: bool,
    pub mode: ShellMode,
    pub prompt: String,
    stdout: RawTerminal<Stdout>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellMode {
    Operational,
    Configuration,
}

impl Shell {
    pub fn new(prompt: String) -> Self {
        let stdout = io::stdout()
            .into_raw_mode()
            .expect("couldn't set stdout into raw mode");
        stdout
            .suspend_raw_mode()
            .expect("couldn't suspend raw mode");
        Shell {
            running: true,
            mode: ShellMode::Operational,
            prompt,
            stdout,
        }
    }

    pub fn process_input(&mut self) -> error::CustomResult<(Command, Vec<String>)> {
        let input = loop {
            self.print_prompt()?;
            let input = self.read_input()?;
            if input.trim().is_empty() {
                continue;
            }
            break input;
        };
        let args: Vec<&str> = input.split_whitespace().collect();

        if let Some(first) = args.first() {
            Ok((
                first.parse()?,
                args.iter().skip(1).map(|s| s.to_string()).collect(),
            ))
        } else {
            panic!("split returned no args");
        }
    }

    pub fn enter_mode(&mut self) -> error::CustomResult<()> {
        match self.mode {
            ShellMode::Operational => {
                self.mode = ShellMode::Configuration;
                Ok(())
            }
            ShellMode::Configuration => Err(ShellError::CannotEnterMode {
                mode: self.mode,
                source: None,
            }
            .into()),
        }
    }

    pub fn exit_mode(&mut self) {
        match self.mode {
            ShellMode::Configuration => self.mode = ShellMode::Operational,
            ShellMode::Operational => self.running = false,
        }
    }
}

impl Shell {
    fn print_prompt(&mut self) -> error::CustomResult<()> {
        write!(self.stdout, "{}", self.prompt)?;
        self.stdout.lock().flush()?;
        Ok(())
    }

    fn read_input(&mut self) -> error::CustomResult<String> {
        let mut stdin = io::stdin().keys();
        let mut buffer = Vec::new();
        let mut cursor_location: usize = 0;
        let mut reading = true;

        self.stdout.activate_raw_mode()?;

        while reading {
            if let Some(Ok(key)) = stdin.next() {
                match key {
                    Key::Ctrl('c') => {
                        reading = false;
                    }
                    Key::Left => {
                        if cursor_location > 0 {
                            write!(self.stdout, "{}", cursor::Left(1))?;
                            cursor_location -= 1;
                        }
                    }
                    Key::Right => {
                        if cursor_location < buffer.len() {
                            write!(self.stdout, "{}", cursor::Right(1))?;
                            cursor_location += 1;
                        }
                    }
                    Key::Backspace => {
                        if cursor_location > 0 {
                            buffer.remove(cursor_location - 1);
                            write!(self.stdout, "{}", cursor::Left(1))?;
                            cursor_location -= 1;
                        }
                    }
                    Key::Char(c) => {
                        if c == '\n' {
                            write!(self.stdout, "\n\r")?;
                            reading = false;
                        } else {
                            write!(self.stdout, "{}", c)?;

                            if cursor_location == buffer.len() {
                                buffer.push(c);
                            } else {
                                buffer.insert(cursor_location, c);

                                for b in buffer[cursor_location + 1..].iter() {
                                    write!(self.stdout, "{}", b)?;
                                }

                                write!(
                                    self.stdout,
                                    "{}",
                                    cursor::Left((buffer.len() - cursor_location - 1) as u16)
                                )?;
                            }
                            cursor_location += 1;
                        }
                    }
                    _ => continue,
                }

                self.stdout.lock().flush()?;
            }
        }

        self.stdout.suspend_raw_mode()?;

        Ok(buffer.into_iter().collect())
    }
}
