use thiserror::Error;

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("Tried to end node at save root (no node to end)")]
    NoNodeToEnd,
    #[error("Tried to set property ('{0}') at save root (no node to set property in)")]
    NoNodeToSetProperty(String),
}
