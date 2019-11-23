pub trait Merge {
    fn merge(&mut self, other: Self, strategy: MergingStrategy) -> anyhow::Result<()>;
}

#[derive(Debug, Copy, Clone)]
pub enum MergingStrategy {
    Ours,
    Theirs,
    Error,
}
