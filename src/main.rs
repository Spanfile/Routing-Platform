#[derive(Debug)]
pub struct Schema {
    value: i32,
}

fn main() {
    let schema = include_str!(concat!(env!("OUT_DIR"), "/schema"));
    println!("{}", schema);
}
