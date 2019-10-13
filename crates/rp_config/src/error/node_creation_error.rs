use rp_error::ErrorTrait;

#[derive(Debug)]
pub struct NodeCreationError {
    pub source: Option<Box<dyn ErrorTrait + 'static>>,
}

impl ErrorTrait for NodeCreationError {
    fn display(&self) -> String {
        String::from("Node creation failed")
    }

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        self.source.as_deref()
    }
}
