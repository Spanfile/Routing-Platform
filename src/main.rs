use common::config::Config;
use common::schema::Schema;

fn main() {
    let binary = include_bytes!(concat!(env!("OUT_DIR"), "/schema"));
    let schema = Schema::from_binary(binary).expect("couldn't load schema from binary");
    schema
        .load_regexes_from_cache()
        .expect("couldn't load regexes from cache");

    schema.print_debug_info();

    let config = Config::from_schema(&schema).expect("couldn't build config from schema");

    println!("{:#?}", config);
}
