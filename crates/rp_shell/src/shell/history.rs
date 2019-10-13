use chrono::prelude::*;
use std::fmt;

#[derive(Debug)]
pub struct HistoryEntry {
    pub command: String,
    pub timestamp: DateTime<Local>,
}

impl HistoryEntry {
    pub fn new(command: String) -> Self {
        HistoryEntry {
            command,
            timestamp: Local::now(),
        }
    }
}

impl fmt::Display for HistoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}",
            self.timestamp.format("%F %H:%M:%S"),
            self.command
        )
    }
}
