use common::schema::Schema;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let pwd = env::current_dir().unwrap();

    let schema_path = Path::new(&pwd).join("schema.yml");
    let dest_path = Path::new(&out_dir).join("schema");

    let schema = Schema::from_file(&schema_path).unwrap();
    let mut dest = File::create(dest_path).unwrap();
    schema.to_file(&mut dest).unwrap();
}