use rp_common::ShellMode;
use thiserror::Error;

#[derive(Debug, Error)]
// TODO: rename
pub enum GeneralError {
    #[error("Invalid shell mode {mode:?} for command '{command}'")]
    InvalidModeForCommand { command: String, mode: ShellMode },
    #[error("There are unapplied changes. Apply them with 'apply', discard them with 'discard' or discard them and exit with 'exit discard'")]
    UnappliedChanges,
}
