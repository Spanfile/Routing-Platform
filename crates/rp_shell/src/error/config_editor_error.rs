use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigEditorError {
    #[error("No such node: {0}")]
    NodeNotFound(String),
    #[error("No such property: {0}")]
    PropertyNotFound(String),
    #[error("Already at top")]
    AlreadyAtTop,
    #[error("Invalid value")]
    ValueError,
    #[error("Ambiguous node name: '{0}' (multiple literal node names)")]
    AmbiguousNodeName(String),
}
