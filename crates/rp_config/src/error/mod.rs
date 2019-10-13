mod constraint_error;
mod node_creation_error;
mod property_error;

pub use constraint_error::ConstraintError;
pub use node_creation_error::NodeCreationError;
pub use property_error::PropertyError;
use rp_error::ErrorTrait;
use std::convert::From;

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug)]
pub enum ConfigError {
    Property(PropertyError),
    NodeCreation(NodeCreationError),
    Constraint(ConstraintError),
}

impl ErrorTrait for ConfigError {
    fn display(&self) -> String {
        match self {
            ConfigError::Property(err) => err.display(),
            ConfigError::NodeCreation(err) => err.display(),
            ConfigError::Constraint(err) => err.display(),
        }
    }

    fn source(&self) -> Option<&(dyn ErrorTrait + 'static)> {
        match self {
            ConfigError::Property(err) => err.source(),
            ConfigError::NodeCreation(err) => err.source(),
            ConfigError::Constraint(err) => err.source(),
        }
    }
}

impl From<PropertyError> for ConfigError {
    fn from(item: PropertyError) -> Self {
        ConfigError::Property(item)
    }
}

impl From<NodeCreationError> for ConfigError {
    fn from(item: NodeCreationError) -> Self {
        ConfigError::NodeCreation(item)
    }
}

impl From<ConstraintError> for ConfigError {
    fn from(item: ConstraintError) -> Self {
        ConfigError::Constraint(item)
    }
}
