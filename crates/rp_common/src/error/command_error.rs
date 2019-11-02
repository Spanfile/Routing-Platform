use std::fmt;
use strum_macros::EnumVariantNames;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("No such command: '{0}'")]
    NotFound(String),
    #[error("Missing argument: {0}, expected {1}")]
    MissingArgument(String, ExpectedValue),
    #[error("Unexpected argument: {0}, expected {1}")]
    UnexpectedArgument(String, ExpectedValue),
}

#[derive(Debug)]
pub enum ExpectedValue {
    Literal(String),
    OneOf(Vec<String>),
}

impl CommandError {
    pub fn not_found(command: String) -> anyhow::Error {
        Self::NotFound(command).into()
    }

    pub fn missing_argument(command: String, expected: ExpectedValue) -> anyhow::Error {
        Self::MissingArgument(command, expected).into()
    }

    pub fn unexpected_argument(command: String, expected: ExpectedValue) -> anyhow::Error {
        Self::UnexpectedArgument(command, expected).into()
    }
}

impl ExpectedValue {
    pub fn from_enum<T>() -> ExpectedValue {}
}

impl fmt::Display for ExpectedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(s) => write!(f, "{}", s),
            Self::OneOf(values) => write!(f, "one of: {}", values.join(", ")),
        }
    }
}
