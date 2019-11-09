pub trait Changeable {
    fn is_clean(&self) -> bool;
    fn apply_changes(&self) -> anyhow::Result<()>;
}
