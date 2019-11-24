use crate::error::MergeError;
use std::mem;

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
    pub fn resolve<T>(&self, ours: &mut T, theirs: T) -> anyhow::Result<()>
    where
        T: std::fmt::Debug + PartialEq,
    {
        match self {
            MergingStrategy::Ours => Ok(()),
            MergingStrategy::Theirs => {
                mem::replace(ours, theirs);
                Ok(())
            }
            MergingStrategy::Error => {
                if *ours != theirs {
                    Err(MergeError::Conflict {
                        this: format!("{:?}", ours),
                        that: format!("{:?}", theirs),
                    }
                    .into())
                } else {
                    Ok(())
                }
            }
        }
    }
}
