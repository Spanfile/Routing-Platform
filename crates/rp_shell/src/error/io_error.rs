use std::backtrace::Backtrace;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("IO error")]
pub struct IoError {
    #[from]
    error: std::io::Error,
    backtrace: Backtrace,
}
