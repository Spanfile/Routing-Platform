use rp_error::ErrorTrait;

#[derive(Debug)]
pub enum PropertyError {
    DefaultResolvingError {
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
    ConstraintNotMet {
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
    NoValueSet {
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
}

impl ErrorTrait for PropertyError {
    fn display(&self) -> String {
        match &self {
            PropertyError::DefaultResolvingError { .. } => {
                String::from("Default value failed to resolve")
            }
            PropertyError::ConstraintNotMet { .. } => String::from("Constraint not met"),
            PropertyError::NoValueSet { .. } => String::from("No value set"),
        }
    }

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        match self {
            PropertyError::DefaultResolvingError { source } => source.as_deref(),
            PropertyError::ConstraintNotMet { source } => source.as_deref(),
            PropertyError::NoValueSet { source } => source.as_deref(),
        }
    }
}
