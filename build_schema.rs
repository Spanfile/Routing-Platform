use common;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("schema");

    let schema = common::Schema {
        value: 16
    };

    fs::write(dest_path, schema.get_binary()).unwrap();
}