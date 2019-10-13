use super::{CommonError, CommonErrorTrait};

#[derive(Debug)]
pub struct IoError {
    pub error: std::io::Error,
    pub source: Option<Box<CommonError>>,
}

impl CommonErrorTrait for IoError {
    fn display(&self) -> String {
        format!("{}", self.error)
    }

    fn source(&self) -> Option<&CommonError> {
        self.source.as_deref()
    }
}
