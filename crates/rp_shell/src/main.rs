#![feature(inner_deref, backtrace, box_syntax)]

mod config_editor;
mod error;
mod shell;

pub use config_editor::ConfigEditor;
use rp_common::{CommandMetadata, ShellMode};
use rp_config::Config;
use rp_log::*;
use rp_schema::Schema;
use shell::{ExecutableCommand, Shell};
use std::{rc::Rc, time::Instant};

fn main() -> anyhow::Result<()> {
    setup_logging()?;

    let binary = include_bytes!(concat!(env!("OUT_DIR"), "/schema"));
    debug!("Schema binary: {} bytes", binary.len());

    let start = Instant::now();
    let schema = Rc::new(Schema::from_binary(binary)?);
    debug!("Schema loaded in {}ms", start.elapsed().as_millis());
    schema.print_debug_info();

    let start = Instant::now();
    let config = Config::from_schema(Rc::downgrade(&schema))?;
    let mut editor = ConfigEditor::new(&config, &schema);
    debug!("Config loaded in {}ms", start.elapsed().as_millis());

    let mut shell = Shell::new()?;

    while shell.running {
        if let Err(e) = process(&mut shell, &mut editor) {
            match e.downcast_ref() {
                Some(error::ShellError::Abort) => {
                    println!();
                }
                Some(error::ShellError::AmbiguousCompletion { .. }) => {
                    println!("\n{}", e);
                }
                _ => error!("{}", e),
            }
        }
    }

    Ok(())
}

fn process(shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
    shell.prompt = get_prompt(shell, editor);
    let command = shell.process_input()?;

    if let Some(required_mode) = command.required_shell_mode() {
        if required_mode != shell.mode {
            return Err(error::GeneralError::InvalidModeForCommand {
                command: format!("{:?}", command),
                mode: shell.mode,
            }
            .into());
        }
    }

    command.run(shell, editor)
}

fn get_prompt(shell: &Shell, editor: &ConfigEditor) -> String {
    match shell.mode {
        ShellMode::Operational => String::from("$ "),
        ShellMode::Configuration => {
            let path = editor.get_current_path();
            format!(
                "\n>{}\n# ",
                if !path.is_empty() {
                    path.join(" ")
                } else {
                    String::from("(top level)")
                }
            )
        }
    }
}
