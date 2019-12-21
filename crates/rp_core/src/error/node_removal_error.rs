use thiserror::Error;

#[derive(Debug, Error)]
#[error("Cannot remove non-removable node '{node}'")]
pub struct NodeRemovalError {
    pub node: String,
}
