use thiserror::Error;

#[derive(Debug, Error)]
#[error("This functionality hasn't been implemented yet ({description})")]
pub struct NotImplemented {
    pub description: String,
}
