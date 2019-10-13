mod command_error;
mod common_error;
mod config_editor_error;
mod general_error;
mod io_error;
mod shell_error;

pub use command_error::CommandError;
pub use common_error::CommonError;
pub use config_editor_error::ConfigEditorError;
pub use general_error::GeneralError;
pub use io_error::IoError;
use rp_error::ErrorTrait;
pub use shell_error::ShellError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Shell(ShellError),
    Command(CommandError),
    General(GeneralError),
    ConfigEditor(ConfigEditorError),
    Common(CommonError),
    Io(IoError),
}

impl ErrorTrait for Error {
    fn display(&self) -> String {
        match self {
            Error::Shell(err) => err.display(),
            Error::Command(err) => err.display(),
            Error::General(err) => err.display(),
            Error::ConfigEditor(err) => err.display(),
            Error::Common(err) => err.display(),
            Error::Io(err) => err.display(),
        }
    }

    fn source(&self) -> Option<&(dyn ErrorTrait)> {
        match self {
            Error::Shell(err) => err.source(),
            Error::Command(err) => err.source(),
            Error::General(err) => err.source(),
            Error::ConfigEditor(err) => err.source(),
            Error::Common(err) => err.source(),
            Error::Io(err) => err.source(),
        }
    }
}

impl From<ShellError> for Error {
    fn from(item: ShellError) -> Self {
        Error::Shell(item)
    }
}

impl From<CommandError> for Error {
    fn from(item: CommandError) -> Self {
        Error::Command(item)
    }
}

impl From<GeneralError> for Error {
    fn from(item: GeneralError) -> Self {
        Error::General(item)
    }
}

impl From<ConfigEditorError> for Error {
    fn from(item: ConfigEditorError) -> Self {
        Error::ConfigEditor(item)
    }
}

impl From<CommonError> for Error {
    fn from(item: CommonError) -> Self {
        Error::Common(item)
    }
}

impl From<IoError> for Error {
    fn from(item: IoError) -> Self {
        Error::Io(item)
    }
}
