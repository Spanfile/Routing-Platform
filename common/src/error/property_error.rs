use super::CommonErrorTrait;

#[derive(Debug)]
pub enum PropertyError {
    DefaultResolvingError,
    ConstraintNotMet,
    NoValueSet,
}

impl CommonErrorTrait for PropertyError {
    fn display(&self) -> String {
        match &self {
            PropertyError::DefaultResolvingError { .. } => {
                String::from("default value failed to resolve")
            }
            PropertyError::ConstraintNotMet => String::from("constraint not met"),
            PropertyError::NoValueSet => String::from("no value set"),
        }
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
