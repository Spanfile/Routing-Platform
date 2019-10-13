use super::{CommonError, CommonErrorTrait};

#[derive(Debug)]
pub struct RegexAutomataError {
    pub error: regex_automata::Error,
    pub source: Option<Box<CommonError>>,
}

impl CommonErrorTrait for RegexAutomataError {
    fn display(&self) -> String {
        format!("regex_automata error: {}", self.error)
    }

    fn source(&self) -> Option<&CommonError> {
        self.source.as_deref()
    }
}
