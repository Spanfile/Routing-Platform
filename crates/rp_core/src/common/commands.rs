use super::ShellMode;

pub trait CommandMetadata {
    fn aliases(&self) -> Vec<&'static str>;
    fn required_shell_mode(&self) -> Option<ShellMode>;
}

pub trait CommandFromArgs
where
    Self: Sized,
{
    fn from_args(args: Vec<String>) -> anyhow::Result<Self>;
}
