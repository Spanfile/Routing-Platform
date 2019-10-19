use super::{ExecutableCommand, Shell, ShellMode};
use crate::{error, ConfigEditor};

#[derive(Debug)]
pub struct Edit;

#[derive(Debug)]
pub struct Up;

#[derive(Debug)]
pub struct Top;

#[derive(Debug)]
pub struct Set;

#[derive(Debug)]
pub struct Remove;

impl ExecutableCommand for Edit {
    fn run(
        &self,
        arguments: Vec<String>,
        _shell: &mut Shell,
        editor: &mut ConfigEditor,
    ) -> anyhow::Result<()> {
        if arguments.is_empty() {
            Err(error::CommandError::MissingArgument(String::from("node")))?
        } else {
            for arg in arguments {
                editor.edit_node(arg)?;
            }

            Ok(())
        }
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["edit"]
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        Some(ShellMode::Configuration)
    }
}

impl ExecutableCommand for Up {
    fn run(
        &self,
        _arguments: Vec<String>,
        _shell: &mut Shell,
        editor: &mut ConfigEditor,
    ) -> anyhow::Result<()> {
        editor.go_up()
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["up"]
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        Some(ShellMode::Configuration)
    }
}

impl ExecutableCommand for Top {
    fn run(
        &self,
        _arguments: Vec<String>,
        _shell: &mut Shell,
        editor: &mut ConfigEditor,
    ) -> anyhow::Result<()> {
        editor.go_top()
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["top"]
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        Some(ShellMode::Configuration)
    }
}

impl ExecutableCommand for Set {
    fn run(
        &self,
        arguments: Vec<String>,
        _shell: &mut Shell,
        editor: &mut ConfigEditor,
    ) -> anyhow::Result<()> {
        let property = arguments
            .get(0)
            .ok_or_else(|| error::CommandError::MissingArgument(String::from("property")))?;
        let value = arguments
            .get(1)
            .ok_or_else(|| error::CommandError::MissingArgument(String::from("value")))?;

        editor.set_property_value(property, value)
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["set"]
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        Some(ShellMode::Configuration)
    }
}

impl ExecutableCommand for Remove {
    fn run(
        &self,
        arguments: Vec<String>,
        _shell: &mut Shell,
        editor: &mut ConfigEditor,
    ) -> anyhow::Result<()> {
        let property = arguments
            .get(0)
            .ok_or_else(|| error::CommandError::MissingArgument(String::from("property")))?;
        let value = arguments.get(1).map(|v| v.as_str());

        editor.remove_property_value(property, value)
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["remove"]
    }

    fn required_shell_mode(&self) -> Option<ShellMode> {
        Some(ShellMode::Configuration)
    }
}
