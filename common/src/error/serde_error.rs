use super::{CommonError, CommonErrorTrait};

#[derive(Debug)]
pub enum SerdeError {
    Json {
        error: serde_json::Error,
        source: Option<Box<CommonError>>,
    },
    Yaml {
        error: serde_yaml::Error,
        source: Option<Box<CommonError>>,
    },
}

impl CommonErrorTrait for SerdeError {
    fn display(&self) -> String {
        match self {
            SerdeError::Json { error, .. } => format!("Serde JSON error: {}", error),
            SerdeError::Yaml { error, .. } => format!("Serde YAML error: {}", error),
        }
    }

    fn source(&self) -> Option<&CommonError> {
        match self {
            SerdeError::Json { source, .. } => source.as_deref(),
            SerdeError::Yaml { source, .. } => source.as_deref(),
        }
    }
}
