use crate::shell::ShellMode;
use rp_error::ErrorTrait;

#[derive(Debug)]
pub enum ShellError {
    CannotEnterMode {
        mode: ShellMode,
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
    Io {
        err: std::io::Error,
        source: Option<Box<dyn ErrorTrait + 'static>>,
    },
}

impl ErrorTrait for ShellError {
    fn display(&self) -> String {
        match self {
            ShellError::CannotEnterMode { mode, .. } => {
                format!("cannot enter mode (from {:?})", mode)
            }
            ShellError::Io { err, .. } => err.to_string(),
        }
    }

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        match self {
            ShellError::CannotEnterMode { source, .. } => source.as_deref(),
            ShellError::Io { source, .. } => source.as_deref(),
        }
    }
}
