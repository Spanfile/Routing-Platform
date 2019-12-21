use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MergeError {
    #[error("Merge conflict: {this} conflicts with {that}")]
    Conflict { this: String, that: String },
}
