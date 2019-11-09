mod common;

use rp_config::Config;
use rp_schema::Schema;
use std::{
    io::{Cursor, Read, Seek, SeekFrom},
    rc::Rc,
};

fn buffer() -> Cursor<Vec<u8>> {
    Cursor::new(Vec::new())
}

#[test]
fn complete_schema() -> anyhow::Result<()> {
    let mut schema = common::get_valid_schema()?;
    schema.build_regex_cache()?;

    let mut buf = buffer();
    schema.to_binary_file(&mut buf)?;

    buf.seek(SeekFrom::Start(0))?;

    let mut bytes = Vec::new();
    buf.read_to_end(&mut bytes)?;
    let schema = Rc::new(Schema::from_binary(&bytes)?);

    Config::from_schema(Rc::downgrade(&schema))?;
    Ok(())
}
