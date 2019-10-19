use thiserror::Error;

#[derive(Debug, Error)]
pub enum FormatError {
    #[error("Empty format string")]
    FormatStringEmpty,
    #[error("ID '{0}' not in context")]
    IdNotInContext(String),
}
