use common::schema::Schema;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("couldn't get OUT_DIR from environment");
    let pwd = env::current_dir().expect("couldn't get current working directory");

    let schema_path = Path::new(&pwd).join("schema.yml");
    let dest_path = Path::new(&out_dir).join("schema");
    let regex_cache_path = Path::new(&out_dir).join("regex_cache");

    let schema_file = File::open(&schema_path).expect("couldn't open YAML schema file");
    let mut schema = Schema::from_yaml_file(&schema_file).expect("couldn't read schema from YAML");

    let validation_errors = schema.validate();
    if !validation_errors.is_empty() {
        println!("----------");
        for err in &validation_errors {
            println!("{}\n----------", err.message);
        }
        panic!();
    }

    let mut schema_dest = File::create(dest_path).expect("couldn't create schema binary file");
    let regex_cache_file = File::create(&regex_cache_path).expect("couldn't open regex cache file");

    schema.to_binary_file(&mut schema_dest).unwrap();
}