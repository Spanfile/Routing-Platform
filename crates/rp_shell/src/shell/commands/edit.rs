use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{error::ExpectedValue, CommandFromArgs, CommandMetadata, ShellMode};

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Edit {
    nodes: Vec<String>,
}

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Up;

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Top;

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Set {
    property: String,
    value: String,
}

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Remove {
    property: String,
    value: Option<String>,
}

impl ExecutableCommand for Edit {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        for arg in &self.nodes {
            editor.edit_node(arg)?;
        }

        Ok(())
    }
}

impl ExecutableCommand for Up {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        editor.go_up()
    }
}

impl ExecutableCommand for Top {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        editor.go_top()
    }
}

impl ExecutableCommand for Set {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        editor.set_property_value(&self.property, &self.value)
    }
}

impl ExecutableCommand for Remove {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        editor.remove_property_value(&self.property, self.value.as_deref())
    }
}
