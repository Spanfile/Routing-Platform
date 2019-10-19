mod commands;
mod completions;
mod history;

use crate::error;
pub use commands::{Command, ExecutableCommand};
use history::HistoryEntry;
use std::io::{self, Stdout, Write};
use termion::{
    self, clear, cursor,
    cursor::DetectCursorPos,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Shell {
    pub running: bool,
    pub mode: ShellMode,
    pub prompt: String,
    stdout: RawTerminal<Stdout>,
    history: Vec<HistoryEntry>,
    history_index: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellMode {
    Operational,
    Configuration,
}

impl Shell {
    pub fn new() -> Self {
        let stdout = io::stdout()
            .into_raw_mode()
            .expect("couldn't set stdout into raw mode");
        stdout
            .suspend_raw_mode()
            .expect("couldn't suspend raw mode");
        Shell {
            running: true,
            mode: ShellMode::Operational,
            prompt: String::from(""),
            stdout,
            history: Vec::new(),
            history_index: None,
        }
    }

    pub fn process_input(&mut self) -> anyhow::Result<(Command, Vec<String>)> {
        let input = loop {
            self.print_prompt()?;
            let input = self.read_input()?;
            if input.trim().is_empty() {
                continue;
            }
            break input;
        };

        let input_args = input.to_owned();
        let args: Vec<&str> = input_args.split_whitespace().collect();

        if let Some(first) = args.first() {
            self.history.push(HistoryEntry::new(input));
            self.history_index = None;
            Ok((
                first.parse()?,
                args.iter().skip(1).map(|s| s.to_string()).collect(),
            ))
        } else {
            panic!("split returned no args");
        }
    }

    pub fn enter_mode(&mut self) -> anyhow::Result<()> {
        match self.mode {
            ShellMode::Operational => {
                self.mode = ShellMode::Configuration;
                Ok(())
            }
            ShellMode::Configuration => Err(error::ShellError::CannotEnterMode(self.mode))?,
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
            write!(self.stdout, "{}\n", entry).map_err(error::IoError::from)?;
        }
        Ok(())
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

impl Shell {
    fn write(&mut self, args: std::fmt::Arguments) -> anyhow::Result<()> {
        write!(self.stdout, "{}", args).map_err(error::IoError::from)?;
        Ok(())
    }

    fn flush(&mut self) -> anyhow::Result<()> {
        self.stdout.lock().flush().map_err(error::IoError::from)?;
        Ok(())
    }

    fn cursor_pos(&mut self) -> anyhow::Result<(u16, u16)> {
        Ok(self.stdout.cursor_pos().map_err(error::IoError::from)?)
    }

    fn activate_raw_mode(&mut self) -> anyhow::Result<()> {
        self.stdout
            .activate_raw_mode()
            .map_err(error::IoError::from)?;
        Ok(())
    }

    fn suspend_raw_mode(&mut self) -> anyhow::Result<()> {
        self.stdout
            .suspend_raw_mode()
            .map_err(error::IoError::from)?;
        Ok(())
    }

    fn print_prompt(&mut self) -> anyhow::Result<()> {
        self.write(format_args!("{}", self.prompt.to_owned()))?;
        self.flush()?;
        Ok(())
    }

    fn read_input(&mut self) -> anyhow::Result<String> {
        let mut stdin = io::stdin().keys();
        let mut buffer = Vec::new();
        let mut cursor_location: usize = 0;
        let mut reading = true;

        self.activate_raw_mode()?;

        while reading {
            if let Some(Ok(key)) = stdin.next() {
                match key {
                    Key::Ctrl('c') => {
                        self.suspend_raw_mode()?;
                        return Err(error::ShellError::Abort)?;
                    }
                    Key::Left => {
                        if cursor_location > 0 {
                            self.write(format_args!("{}", cursor::Left(1)))?;
                            cursor_location -= 1;
                        }
                    }
                    Key::Right => {
                        if cursor_location < buffer.len() {
                            self.write(format_args!("{}", cursor::Right(1)))?;
                            cursor_location += 1;
                        }
                    }
                    Key::Up => {
                        let (cursor_x, cursor_y) = self.cursor_pos()?;
                        let index = match self.history_index {
                            Some(i) => {
                                if i > 0 {
                                    self.history_index = Some(i - 1);
                                    i - 1
                                } else {
                                    i
                                }
                            }
                            None => {
                                let i = self.history.len() - 1;
                                self.history_index = Some(i);
                                i
                            }
                        };

                        buffer = self.history.get(index).unwrap().command.chars().collect();

                        self.write(format_args!(
                            "{}{}",
                            cursor::Goto(cursor_x - cursor_location as u16, cursor_y),
                            clear::UntilNewline
                        ))?;

                        for b in buffer.iter() {
                            self.write(format_args!("{}", b))?;
                        }

                        cursor_location = buffer.len();
                    }
                    Key::Down => {
                        let (cursor_x, cursor_y) = self.cursor_pos()?;
                        let index = match self.history_index {
                            Some(i) => {
                                if i == self.history.len() - 1 {
                                    self.history_index = None;
                                } else {
                                    self.history_index = Some(i + 1);
                                }
                                self.history_index
                            }
                            None => None,
                        };

                        self.write(format_args!(
                            "{}{}",
                            cursor::Goto(cursor_x - cursor_location as u16, cursor_y),
                            clear::UntilNewline
                        ))?;

                        if let Some(i) = index {
                            buffer = self.history.get(i).unwrap().command.chars().collect();

                            for b in buffer.iter() {
                                self.write(format_args!("{}", b))?;
                            }
                        } else {
                            buffer.clear();
                        }

                        cursor_location = buffer.len();
                    }
                    Key::Backspace => {
                        if cursor_location > 0 {
                            buffer.remove(cursor_location - 1);
                            let (cursor_x, cursor_y) = self.cursor_pos()?;

                            self.write(format_args!(
                                "{}{}",
                                cursor::Goto(cursor_x - cursor_location as u16, cursor_y),
                                clear::UntilNewline
                            ))?;

                            for b in buffer.iter() {
                                self.write(format_args!("{}", b))?;
                            }

                            self.write(format_args!("{}", cursor::Goto(cursor_x - 1, cursor_y)))?;

                            cursor_location -= 1;
                        }
                    }
                    Key::Delete => {
                        if cursor_location < buffer.len() {
                            buffer.remove(cursor_location);
                            let (cursor_x, cursor_y) = self.cursor_pos()?;

                            self.write(format_args!("{}", clear::UntilNewline))?;

                            for b in buffer.iter() {
                                self.write(format_args!("{}", b))?;
                            }

                            self.write(format_args!("{}", cursor::Goto(cursor_x, cursor_y)))?;
                        }
                    }
                    Key::Char(c) => {
                        if c == '\n' {
                            self.write(format_args!("\n\r"))?;
                            reading = false;
                        } else {
                            self.write(format_args!("{}", c))?;

                            if cursor_location == buffer.len() {
                                buffer.push(c);
                            } else {
                                buffer.insert(cursor_location, c);

                                for b in buffer[cursor_location + 1..].iter() {
                                    self.write(format_args!("{}", b))?;
                                }

                                self.write(format_args!(
                                    "{}",
                                    cursor::Left((buffer.len() - cursor_location - 1) as u16)
                                ))?;
                            }

                            cursor_location += 1;
                        }
                    }
                    _ => continue,
                }

                self.flush()?;
            }
        }

        self.suspend_raw_mode()?;
        Ok(buffer.into_iter().collect())
    }
}