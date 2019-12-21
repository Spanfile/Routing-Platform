use rp_core::common::ShellMode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("Cannot enter mode (from {0:?}")]
    CannotEnterMode(ShellMode),
    #[error("Abort")]
    Abort,
    #[error("{}", .0.join(" "))]
    AmbiguousCompletion(Vec<&'static str>),
}
