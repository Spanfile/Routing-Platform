use super::ShellMode;

pub trait CommandMetadata {
    fn aliases(&self) -> Vec<&str>;
    fn required_shell_mode(&self) -> Option<ShellMode>;
}
