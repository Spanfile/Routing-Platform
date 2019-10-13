use super::{CommonError, CommonErrorTrait};

#[derive(Debug)]
pub enum PropertyError {
    DefaultResolvingError { source: Option<Box<CommonError>> },
    ConstraintNotMet { source: Option<Box<CommonError>> },
    NoValueSet { source: Option<Box<CommonError>> },
}

impl CommonErrorTrait for PropertyError {
    fn display(&self) -> String {
        match &self {
            PropertyError::DefaultResolvingError { .. } => {
                String::from("Default value failed to resolve")
            }
            PropertyError::ConstraintNotMet { .. } => String::from("Constraint not met"),
            PropertyError::NoValueSet { .. } => String::from("No value set"),
        }
    }

    fn source(&self) -> Option<&CommonError> {
        match self {
            PropertyError::DefaultResolvingError { source } => source.as_deref(),
            PropertyError::ConstraintNotMet { source } => source.as_deref(),
            PropertyError::NoValueSet { source } => source.as_deref(),
        }
    }
}
