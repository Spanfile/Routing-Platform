#![feature(inner_deref)]
#![feature(backtrace)]

extern crate chrono;

mod config_editor;
mod error;
mod shell;

pub use config_editor::ConfigEditor;
use rp_config::Config;
use rp_schema::Schema;
use shell::{ExecutableCommand, Shell, ShellMode};
use std::{rc::Rc, time::Instant};

fn main() {
    let binary = include_bytes!(concat!(env!("OUT_DIR"), "/schema"));

    let start = Instant::now();
    let schema = Rc::new(Schema::from_binary(binary).expect("couldn't load schema from binary"));
    println!("Schema loaded in {}ms", start.elapsed().as_millis());

    schema.print_debug_info();
    // println!("{:#?}", schema);

    let start = Instant::now();
    let config =
        Config::from_schema(Rc::downgrade(&schema)).expect("couldn't build config from schema");
    let mut editor = ConfigEditor::new(&config, &schema);
    println!("Config loaded in {}ms", start.elapsed().as_millis());

    let mut shell = Shell::new();

    while shell.running {
        if let Err(e) = process(&mut shell, &mut editor) {
            if let Some(error::ShellError::Abort) = e.downcast_ref() {
                println!();
                continue;
            }
            println!("{}", e);
        }
    }
}

fn process(shell: &mut Shell, editor: &mut ConfigEditor) -> anyhow::Result<()> {
    shell.prompt = get_prompt(shell, editor);
    let (command, args) = shell.process_input()?;

    if let Some(required_mode) = command.required_shell_mode() {
        if required_mode != shell.mode {
            return Err(error::GeneralError::InvalidModeForCommand {
                command: format!("{:?}", command),
                mode: shell.mode,
            }
            .into());
        }
    }

    command.run(args, shell, editor)
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
