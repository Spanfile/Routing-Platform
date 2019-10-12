use super::CommonErrorTrait;
use std::error::Error;

#[derive(Debug)]
pub enum SerdeError {
    Json(serde_json::Error),
    Yaml(serde_yaml::Error),
}

impl CommonErrorTrait for SerdeError {
    fn display(&self) -> String {
        match self {
            SerdeError::Json(e) => format!("Serde JSON error: {}", e),
            SerdeError::Yaml(e) => format!("Serde YAML error: {}", e),
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SerdeError::Json(e) => e.source(),
            SerdeError::Yaml(e) => e.source(),
        }
    }
}
