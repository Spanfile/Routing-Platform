use rp_error::ErrorTrait;

#[derive(Debug)]
pub struct CommonError {
    pub source: Option<Box<dyn ErrorTrait>>,
}

impl ErrorTrait for CommonError {
    fn display(&self) -> String {
        String::from("Common error")
    }

    fn source(&self) -> Option<&dyn ErrorTrait> {
        self.source.as_deref()
    }
}
