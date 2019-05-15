mod config_editor;

use common::config::Config;
use common::schema::Schema;
use config_editor::ConfigEditor;

fn main() {
    let binary = include_bytes!(concat!(env!("OUT_DIR"), "/schema"));
    let schema = Schema::from_binary(binary).expect("couldn't load schema from binary");
    schema
        .load_regexes_from_cache()
        .expect("couldn't load regexes from cache");

    schema.print_debug_info();

    let config = Config::from_schema(&schema).expect("couldn't build config from schema");
    let mut editor = ConfigEditor::new(&config, &schema);

    println!("{:?}", editor.get_available_nodes_and_properties());
    editor.edit_node(String::from("interfaces")).unwrap();
    println!("{:?}", editor.get_available_nodes_and_properties());
    editor.edit_node(String::from("ethernet eth0")).unwrap();
    println!("{:?}", editor.get_available_nodes_and_properties());
    println!("{:?}", editor.get_property_values(None));
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

    println!("{:#?}", config);
}
