use rp_error::ErrorTrait;
use std::convert::From;

#[derive(Debug)]
pub enum SerdeError {
    Json {
        error: serde_json::Error,
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
    Yaml {
        error: serde_yaml::Error,
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
}

impl ErrorTrait for SerdeError {
    fn display(&self) -> String {
        match self {
            SerdeError::Json { error, .. } => format!("Serde JSON error: {}", error),
            SerdeError::Yaml { error, .. } => format!("Serde YAML error: {}", error),
        }
    }

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        match self {
            SerdeError::Json { source, .. } => source.as_deref(),
            SerdeError::Yaml { source, .. } => source.as_deref(),
        }
    }
}

impl From<serde_json::Error> for SerdeError {
    fn from(item: serde_json::Error) -> Self {
        SerdeError::Json {
            error: item,
            source: None,
        }
    }
}

impl From<serde_yaml::Error> for SerdeError {
    fn from(item: serde_yaml::Error) -> Self {
        SerdeError::Yaml {
            error: item,
            source: None,
        }
    }
}
