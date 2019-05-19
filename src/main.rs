mod config_editor;

use common::config::Config;
use common::schema::Schema;
use config_editor::ConfigEditor;
use std::time::Instant;

fn main() {
    let binary = include_bytes!(concat!(env!("OUT_DIR"), "/schema"));

    let start = Instant::now();
    let schema = Schema::from_binary(binary).expect("couldn't load schema from binary");
    schema
        .load_regexes_from_cache()
        .expect("couldn't load regexes from cache");
    println!("Schema loaded in {}ms", start.elapsed().as_millis());

    schema.print_debug_info();

    let start = Instant::now();
    let config = Config::from_schema(&schema).expect("couldn't build config from schema");
    let mut editor = ConfigEditor::new(&config, &schema);
    println!("Config loaded in {}ms", start.elapsed().as_millis());

    editor.edit_node(String::from("interfaces")).unwrap();
    editor.edit_node(String::from("ethernet eth0")).unwrap();
    editor.edit_node(String::from("vlan")).unwrap();
    println!("{:?}", editor.get_available_nodes().unwrap());
    editor.edit_node(String::from("10")).unwrap();
    editor
        .set_property_value(String::from("address"), String::from("192.168.0.1/24"))
        .unwrap();

    editor.pretty_print_current_node();
    editor.pretty_print_config();
}
