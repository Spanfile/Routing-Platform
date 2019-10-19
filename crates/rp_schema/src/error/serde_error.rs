use std::backtrace::Backtrace;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SerdeError {
    #[error("Serde JSON error")]
    Json {
        #[from]
        error: serde_json::Error,
        backtrace: Backtrace,
    },
    #[error("Serde YAML error")]
    Yaml {
        #[from]
        error: serde_yaml::Error,
        backtrace: Backtrace,
    },
}
