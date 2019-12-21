#![feature(inner_deref, backtrace, box_syntax)]

mod config_editor;
pub mod error;
mod shell;

pub use config_editor::ConfigEditor;
use rp_core::{
    common::{CommandMetadata, ShellMode},
    config::Config,
    plugin::PluginManager,
    schema::Schema,
};
use rp_log::*;
use shell::{ExecutableCommand, Shell};
use std::{rc::Rc, time::Instant};

pub async fn run() -> anyhow::Result<()> {
    setup_logging()?;

    let binary = include_bytes!(concat!(env!("OUT_DIR"), "/schema"));
    trace!("Schema binary: {} bytes", binary.len());

    let start = Instant::now();
    let schema = Rc::new(Schema::from_binary(binary)?);
    debug!("Schema loaded in {}ms", start.elapsed().as_millis());
    schema.print_trace_info();

    let start = Instant::now();
    let config = Config::from_schema(Rc::downgrade(&schema))?;
    let mut editor = ConfigEditor::new(&config, &schema);
    debug!("Config created in {}ms", start.elapsed().as_millis());

    let start = Instant::now();
    let mut plugin_manager = PluginManager::new();
    unsafe {
        plugin_manager.load_plugin("librp_system.so")?;
    }
    debug!("Plugins loaded in {}ms", start.elapsed().as_millis());

    let start = Instant::now();
    editor.load()?;
    debug!("Config loaded in {}ms", start.elapsed().as_millis());

    let mut shell = Shell::new()?;

    while shell.running {
        if let Err(e) = process(&mut shell, &mut editor).await {
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

async fn process(shell: &mut Shell, editor: &mut ConfigEditor<'_>) -> anyhow::Result<()> {
    shell.prompt = get_prompt(shell, editor);
    let command = shell.process_input().await?;

    if let Some(required_mode) = command.required_shell_mode() {
        if required_mode != shell.mode {
            return Err(error::GeneralError::InvalidModeForCommand {
                command: format!("{:?}", command),
                mode: shell.mode,
            }
            .into());
        }
    }

    let start = Instant::now();
    let result = command.run(shell, editor);
    debug!("Command execution took {}ms", start.elapsed().as_millis());
    result
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
