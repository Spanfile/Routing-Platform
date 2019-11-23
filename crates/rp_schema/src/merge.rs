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
        T: std::fmt::Debug,
    {
        match self {
            MergingStrategy::Ours => Ok(ours),
            MergingStrategy::Theirs => Ok(theirs),
            MergingStrategy::Error => Err(MergeError::Conflict {
                this: format!("{:?}", ours),
                that: format!("{:?}", theirs),
            }
            .into()),
        }
    }
}
