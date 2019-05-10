use common::schema::Schema;
use std::path::Path;

fn main() {
    let schema = Schema::from_file(&Path::new(env!("OUT_DIR")).join("schema")).unwrap();
    println!("{:?}", schema);
}
