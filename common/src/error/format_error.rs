use super::{CommonError, CommonErrorTrait};

#[derive(Debug)]
pub enum FormatError {
    FormatStringEmpty {
        source: Option<Box<CommonError>>,
    },
    IdNotInContext {
        id: String,
        source: Option<Box<CommonError>>,
    },
}

impl CommonErrorTrait for FormatError {
    fn display(&self) -> String {
        match &self {
            FormatError::FormatStringEmpty { .. } => String::from("Empty format string"),
            FormatError::IdNotInContext { id, .. } => format!("ID '{}' not in context", id),
        }
    }

    fn source(&self) -> Option<&CommonError> {
        match self {
            FormatError::FormatStringEmpty { source } => source.as_deref(),
            FormatError::IdNotInContext { source, .. } => source.as_deref(),
        }
    }
}
