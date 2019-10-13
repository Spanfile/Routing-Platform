use rp_error::ErrorTrait;

#[derive(Debug)]
pub struct CommonError {
    pub source: Option<Box<dyn ErrorTrait + 'static>>,
}

impl ErrorTrait for CommonError {
    fn display(&self) -> String {
        String::from("Common error")
    }

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        self.source.as_deref()
    }
}
