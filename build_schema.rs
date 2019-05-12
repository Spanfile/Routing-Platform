use common::schema::Schema;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let pwd = env::current_dir().unwrap();

    let schema_path = Path::new(&pwd).join("schema.yml");
    let dest_path = Path::new(&out_dir).join("schema");

    let file = File::open(&schema_path).unwrap();
    let schema = Schema::from_yaml_file(&file).unwrap();

    let validation_errors = schema.validate();
    if !validation_errors.is_empty() {
        println!("----------");
        for err in &validation_errors {
            println!("{}\n----------", err.message);
        }
        panic!();
    }

    let mut dest = File::create(dest_path).unwrap();
    schema.to_binary_file(&mut dest).unwrap();
}