use rp_error::ErrorTrait;

#[derive(Debug)]
pub struct QueryError {
    pub source: Option<Box<dyn ErrorTrait + 'static>>,
}

impl ErrorTrait for QueryError {
    fn display(&self) -> String {
        String::from("Error in query")
    }

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        self.source.as_deref()
    }
}
