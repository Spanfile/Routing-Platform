use super::{ExecutableCommand, Shell};
use crate::ConfigEditor;
use command_metadata::command;
use rp_common::{CommandFromArgs, CommandMetadata, ShellMode};
use strum::{EnumString, EnumVariantNames, VariantNames};

#[command]
#[derive(Debug)]
pub struct Show {
    args: Vec<String>,
}

#[strum(serialize_all = "lowercase")]
#[derive(Debug, EnumString, EnumVariantNames)]
enum ShowArgument {
    Configuration,
    Schema,
}

impl ExecutableCommand for Show {
    fn run(&self, shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
        match shell.mode {
            ShellMode::Operational => match self.args.first() {
                Some(a) => match a.parse()? {
                    ShowArgument::Configuration => {
                        editor.pretty_print_config();
                        Ok(())
                    }
                    ShowArgument::Schema => Ok(()),
                },
                None => Err(rp_common::error::CommandError::missing_argument(
                    "item",
                    rp_common::error::ExpectedValue::from_enum::<ShowArgument>(),
                )),
            },
            ShellMode::Configuration => traverse(editor, &self.args),
        }
    }
}

fn traverse(editor: &mut ConfigEditor, nodes: &[String]) -> anyhow::Result<()> {
    if nodes.is_empty() {
        editor.pretty_print_current_node();
    } else {
        editor.edit_node(
            nodes
                .first()
                .expect("no first node after checking emptyness"),
        )?;
        traverse(editor, &nodes[1..])?;
        editor.go_up()?;
    }
    Ok(())
}
