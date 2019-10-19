use rp_common::ShellMode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeneralError {
    #[error("Invalid shell mode {mode:?} for command '{command}'")]
    InvalidModeForCommand { command: String, mode: ShellMode },
}
