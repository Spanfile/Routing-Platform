use rp_error::ErrorTrait;

#[derive(Debug)]
pub enum FormatError {
    FormatStringEmpty {
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
    IdNotInContext {
        id: String,
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
}

impl ErrorTrait for FormatError {
    fn display(&self) -> String {
        match &self {
            FormatError::FormatStringEmpty { .. } => String::from("Empty format string"),
            FormatError::IdNotInContext { id, .. } => format!("ID '{}' not in context", id),
        }
    }

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        match self {
            FormatError::FormatStringEmpty { source } => source.as_deref(),
            FormatError::IdNotInContext { source, .. } => source.as_deref(),
        }
    }
}
