use super::ShellMode;

pub trait CommandMetadata
where
    Self: Sized,
{
    fn from_args(args: Vec<String>) -> anyhow::Result<Self>;
    fn aliases(&self) -> Vec<&str>;
    fn required_shell_mode(&self) -> Option<ShellMode>;
}
