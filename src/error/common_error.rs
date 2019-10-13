use super::{Error, ErrorTrait};

#[derive(Debug)]
pub struct CommonError {
    pub source: common::error::CommonError,
}

impl ErrorTrait for CommonError {
    fn display(&self) -> String {
        format!("{}", self.source)
    }

    fn source(&self) -> Option<&Error> {
        None
    }
}
