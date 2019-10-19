use rp_common::ShellMode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("Cannot enter mode (from {0:?}")]
    CannotEnterMode(ShellMode),
    #[error("Abort")]
    Abort,
}
