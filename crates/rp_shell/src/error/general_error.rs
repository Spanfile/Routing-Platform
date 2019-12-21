use rp_core::common::ShellMode;
use thiserror::Error;

#[derive(Debug, Error)]
// TODO: rename
pub enum GeneralError {
    #[error("Invalid shell mode {mode:?} for command '{command}'")]
    InvalidModeForCommand { command: String, mode: ShellMode },
}
