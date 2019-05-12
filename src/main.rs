use common::schema::Schema;
use std::path::Path;
use std::fs::File;

fn main() {
    let path = Path::new(env!("OUT_DIR")).join("schema");
    let file = File::open(&path).unwrap();
    let schema = Schema::from_binary_file(&file).unwrap();
    println!("{:?}", schema);
}
