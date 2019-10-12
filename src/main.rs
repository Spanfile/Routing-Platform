#![feature(inner_deref)]

mod config_editor;
mod error;
mod general_error;
mod shell;

use common::{config::Config, schema::Schema};
pub use config_editor::{ConfigEditor, ConfigEditorError};
use general_error::GeneralError;
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
            println!("{}", e);
        }
    }

    // editor.pretty_print_config();

    // editor.edit_node(String::from("interfaces")).unwrap();
    // editor.edit_node(String::from("ethernet")).unwrap();
    // editor.edit_node(String::from("eth0")).unwrap();
    // editor.edit_node(String::from("vlan")).unwrap();
    // editor.edit_node(String::from("10")).unwrap();

    // editor
    //     .set_property_value(String::from("description"),
    // String::from("test"))     .unwrap();

    // editor.go_up().unwrap();
    // editor.go_up().unwrap();

    // println!(
    //     "{:?}",
    //     editor.get_property_values(Some(String::from("description")))
    // );
    // editor
    //     .set_property_value(String::from("description"),
    // String::from("test"))     .unwrap();
    // editor
    //     .set_property_value(String::from("mtu"), String::from("8000"))
    //     .unwrap();
    // editor
    //     .remove_property_value(String::from("mtu"), None)
    //     .unwrap();

    // editor.go_up().unwrap();
    // editor.go_up().unwrap();
    // editor.go_up().unwrap();

    // editor.edit_node(String::from("system")).unwrap();
    // editor.edit_node(String::from("ntp")).unwrap();
    // println!("{:?}", editor.get_available_nodes_and_properties());
    // editor
    //     .remove_property_value(String::from("server"),
    // Some(String::from("4.pool.ntp.org")))     .unwrap();
    // editor.pretty_print_current_node();
    // editor
    //     .remove_property_value(String::from("server"), None)
    //     .unwrap();

    // // editor.pretty_print_current_node();
    // editor.pretty_print_config();
}

fn process(shell: &mut Shell, editor: &mut ConfigEditor) -> error::CustomResult<()> {
    shell.prompt = get_prompt(shell, editor);
    let (command, args) = shell.process_input()?;

    if let Some(required_mode) = command.required_shell_mode() {
        if required_mode != shell.mode {
            return Err(GeneralError::InvalidModeForCommand {
                command: format!("{:?}", command),
                mode: shell.mode,
                source: None,
            }
            .into());
        }
    }

    command.run(args, shell, editor)
}

fn get_prompt(shell: &Shell, editor: &ConfigEditor) -> String {
    match shell.mode {
        ShellMode::Operational => String::from("& "),
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
