use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("No such node: {0}")]
    NoSuchNode(String),
    #[error("No such property: {0}")]
    NoSuchProperty(String),
    #[error("Tried to end node at load root (no node to end)")]
    NoNodeToEnd,
    #[error("Tried to get property ('{0}') at load root (no node to get from)")]
    NoNodeToGetProperty(String),
}
