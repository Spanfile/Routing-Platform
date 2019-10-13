use rp_error::ErrorTrait;

#[derive(Debug)]
pub struct QueryError {
    pub source: Option<Box<dyn ErrorTrait>>,
}

impl ErrorTrait for QueryError {
    fn display(&self) -> String {
        String::from("Error in query")
    }

    fn source(&self) -> Option<&(dyn ErrorTrait)> {
        self.source.as_deref()
    }
}
