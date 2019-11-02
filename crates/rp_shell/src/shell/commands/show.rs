use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{CommandFromArgs, CommandMetadata, ShellMode};

#[command(required_shell_mode = "Configuration")]
#[derive(Debug)]
pub struct Show {
    nodes: Vec<String>,
}

impl ExecutableCommand for Show {
    fn run(&self, _shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        traverse(editor, &self.nodes)
    }
}

fn traverse(editor: &mut ConfigEditor, nodes: &Vec<String>) -> anyhow::Result<()> {
    if nodes.is_empty() {
        editor.pretty_print_current_node();
    } else {
        editor.edit_node(
            nodes
                .first()
                .expect("no first node after checking emptyness"),
        )?;
        traverse(editor, &nodes.iter().skip(1).cloned().collect())?;
        editor.go_up()?;
    }
    Ok(())
}
