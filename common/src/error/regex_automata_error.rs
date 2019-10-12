use super::CommonErrorTrait;
use std::error::Error;

#[derive(Debug)]
pub struct RegexAutomataError {
    pub error: regex_automata::Error,
}

impl CommonErrorTrait for RegexAutomataError {
    fn display(&self) -> String {
        format!("regex_automata error: {}", self.error)
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}
