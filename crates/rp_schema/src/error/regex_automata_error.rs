use rp_error::ErrorTrait;

#[derive(Debug)]
pub struct RegexAutomataError {
    pub error: regex_automata::Error,
    pub source: Option<Box<dyn ErrorTrait>>,
}

impl ErrorTrait for RegexAutomataError {
    fn display(&self) -> String {
        format!("regex_automata error: {}", self.error)
    }

    fn source(&self) -> Option<&dyn ErrorTrait> {
        self.source.as_deref()
    }
}

impl From<regex_automata::Error> for RegexAutomataError {
    fn from(item: regex_automata::Error) -> Self {
        RegexAutomataError {
            error: item,
            source: None,
        }
    }
}
