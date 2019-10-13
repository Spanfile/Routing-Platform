use rp_error::ErrorTrait;

#[derive(Debug)]
pub struct IoError {
    pub error: std::io::Error,
    pub source: Option<Box<dyn ErrorTrait + 'static>>,
}

impl ErrorTrait for IoError {
    fn display(&self) -> String {
        format!("{}", self.error)
    }

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        self.source.as_deref()
    }
}

impl From<std::io::Error> for IoError {
    fn from(item: std::io::Error) -> Self {
        IoError {
            error: item,
            source: None,
        }
    }
}
