use common::Schema;

fn main() {
    let schema = Schema::from_binary(String::from(include_str!(concat!(env!("OUT_DIR"), "/schema"))));
    println!("{:?}", schema);
}
