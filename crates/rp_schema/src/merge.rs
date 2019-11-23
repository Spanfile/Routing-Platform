use crate::error::MergeError;

pub trait Merge {
    fn merge(&mut self, other: Self, strategy: MergingStrategy) -> anyhow::Result<()>;
}

#[derive(Debug, Copy, Clone)]
pub enum MergingStrategy {
    Ours,
    Theirs,
    Error,
}

impl MergingStrategy {
    pub fn resolve<T>(&self, ours: T, theirs: T) -> anyhow::Result<T>
    where
        T: ToString,
    {
        match self {
            MergingStrategy::Ours => Ok(ours),
            MergingStrategy::Theirs => Ok(theirs),
            MergingStrategy::Error => Err(MergeError::Conflict {
                this: ours.to_string(),
                that: theirs.to_string(),
            }
            .into()),
        }
    }
}
