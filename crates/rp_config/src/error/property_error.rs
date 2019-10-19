use thiserror::Error;

#[derive(Debug, Error)]
pub enum PropertyError {
    #[error("Constraint not met")]
    ConstraintNotMet,
    #[error("No value set")]
    NoValueSet,
}
