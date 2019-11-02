use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{CommandFromArgs, CommandMetadata, ShellMode};
use strum::{EnumVariantNames, VariantNames};

#[command]
#[derive(Debug)]
pub struct Show {
    args: Vec<String>,
}

#[derive(Debug, EnumVariantNames)]
enum ShowArgument {
    Configuration,
    Schema,
}

impl ExecutableCommand for Show {
    fn run(&self, shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        match shell.mode {
            ShellMode::Operational => match self.args.first() {
                Some(_) => Ok(()),
                None => Err(rp_common::error::CommandError::missing_argument(
                    "show",
                    rp_common::error::ExpectedValue::from_enum::<ShowArgument>(),
                )),
            },
            ShellMode::Configuration => traverse(editor, &self.args),
        }
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
