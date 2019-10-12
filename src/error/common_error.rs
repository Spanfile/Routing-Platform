use super::{Error, ErrorTrait};
use common::error::CommonErrorTrait;

#[derive(Debug)]
pub struct CommonError {
    pub source: common::error::CommonError,
}

impl ErrorTrait for CommonError {
    fn display(&self) -> String {
        self.source.display()
    }

    fn source(&self) -> Option<&Error> {
        None
    }
}
