use common::schema::Schema;
use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new(env!("OUT_DIR")).join("schema");
    let file = File::open(&path).unwrap();
    let schema = Schema::from_binary_file(&file).unwrap();
    schema
        .load_regexes_from_cache()
        .expect("couldn't load regexes from cache");

    schema.print_debug_info();
}
