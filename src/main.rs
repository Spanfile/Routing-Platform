mod config_editor;

use common::{config::Config, schema::Schema};
use config_editor::ConfigEditor;
use std::time::Instant;

fn main() {
    let binary = include_bytes!(concat!(env!("OUT_DIR"), "/schema"));

    let start = Instant::now();
    let schema = Schema::from_binary(binary).expect("couldn't load schema from binary");
    println!("Schema loaded in {}ms", start.elapsed().as_millis());

    schema.print_debug_info();

    let start = Instant::now();
    let config = Config::from_schema(&schema).expect("couldn't build config from schema");
    let mut editor = ConfigEditor::new(&config, &schema);
    println!("Config loaded in {}ms", start.elapsed().as_millis());

    editor.edit_node(String::from("interfaces")).unwrap();
    editor.edit_node(String::from("ethernet eth0")).unwrap();
    editor.edit_node(String::from("vlan")).unwrap();
    editor.edit_node(String::from("10")).unwrap();

    editor.go_up().unwrap();
    editor.go_up().unwrap();

    println!(
        "{:?}",
        editor.get_property_values(Some(String::from("description")))
    );
    editor
        .set_property_value(String::from("description"), String::from("test"))
        .unwrap();
    editor
        .set_property_value(String::from("mtu"), String::from("8000"))
        .unwrap();
    editor
        .remove_property_value(String::from("mtu"), None)
        .unwrap();

    editor.go_up().unwrap();
    editor.go_up().unwrap();

    editor.edit_node(String::from("system")).unwrap();
    editor.edit_node(String::from("ntp")).unwrap();
    println!("{:?}", editor.get_available_nodes_and_properties());
    editor
        .remove_property_value(String::from("server"), Some(String::from("4.pool.ntp.org")))
        .unwrap();
    editor.pretty_print_current_node();
    editor
        .remove_property_value(String::from("server"), None)
        .unwrap();

    // editor.pretty_print_current_node();
    editor.pretty_print_config();
}
