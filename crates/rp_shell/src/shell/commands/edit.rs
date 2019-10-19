use super::{ExecutableCommand, Shell};
use crate::{error, ConfigEditor};
use command_metadata::command;
use rp_common::{CommandMetadata, ShellMode};

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Edit;

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Up;

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Top;

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Set;

#[command(required_shell_mode = "Configuration")]
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
            Err(error::CommandError::MissingArgument(String::from("node")).into())
        } else {
            for arg in arguments {
                editor.edit_node(arg)?;
            }

            Ok(())
        }
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
}
