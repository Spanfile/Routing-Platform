use std::fmt;

pub trait ErrorTrait: fmt::Debug {
    fn display(&self) -> String;
    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)>;
}

impl fmt::Display for dyn ErrorTrait {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display())?;
        if let Some(source) = self.source() {
            write!(f, "\n-> {}", source)
        } else {
            Ok(())
        }
    }
}

impl std::error::Error for dyn ErrorTrait {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
