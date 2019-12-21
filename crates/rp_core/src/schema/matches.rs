pub trait Matches {
    fn matches(&self, value: &str) -> anyhow::Result<bool>;
}
