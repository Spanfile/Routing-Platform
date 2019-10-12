use common::schema::Schema;
use std::{env, fs::File, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").expect("couldn't get OUT_DIR from environment");
    let pwd = env::current_dir().expect("couldn't get current working directory");

    let schema_path = Path::new(&pwd).join("schema.yml");
    let dest_path = Path::new(&out_dir).join("schema");

    let schema_file = File::open(&schema_path).expect("couldn't open YAML schema file");
    let mut schema = Schema::from_yaml_file(&schema_file).expect("couldn't read schema from YAML");

    println!("{:#?}", schema);

    if let Err(e) = schema.validate() {
        println!("schema validation failed\n{}", e);
        panic!();
    }

    let mut schema_dest = File::create(dest_path).expect("couldn't create schema binary file");

    schema
        .build_regex_cache()
        .expect("couldn't build regex cache");
    schema
        .to_binary_file(&mut schema_dest)
        .expect("couldn't serialise schema to binary file");
}
