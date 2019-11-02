use std::fmt;
use strum::VariantNames;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("No such command: '{0}'")]
    NotFound(String),
    #[error("Missing argument: {0}, expected {1}")]
    MissingArgument(&'static str, ExpectedValue),
    #[error("Unexpected argument: {0}, expected {1}")]
    UnexpectedArgument(String, ExpectedValue),
}

#[derive(Debug)]
pub enum ExpectedValue {
    Literal(&'static str),
    OneOf(&'static [&'static str]),
}

impl CommandError {
    pub fn not_found(command: String) -> anyhow::Error {
        Self::NotFound(command).into()
    }

    pub fn missing_argument(argument: &'static str, expected: ExpectedValue) -> anyhow::Error {
        Self::MissingArgument(argument, expected).into()
    }

    pub fn unexpected_argument(argument: String, expected: ExpectedValue) -> anyhow::Error {
        Self::UnexpectedArgument(argument, expected).into()
    }
}

impl ExpectedValue {
    pub fn from_enum<T: VariantNames>() -> ExpectedValue {
        Self::OneOf(T::variants())
    }
}

impl fmt::Display for ExpectedValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(s) => write!(f, "{}", s),
            Self::OneOf(values) => write!(f, "one of: {}", values.join(", ")),
        }
    }
}
