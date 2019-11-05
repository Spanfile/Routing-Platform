mod common;

use rp_config::Config;
use rp_schema::Schema;
use std::{
    io::{Read, Seek, SeekFrom},
    rc::Rc,
};
use tempfile::tempfile;

#[test]
fn complete_schema() -> anyhow::Result<()> {
    let mut schema = common::get_valid_schema()?;
    schema.build_regex_cache()?;

    let mut file = tempfile()?;
    schema.to_binary_file(&mut file)?;

    file.seek(SeekFrom::Start(0))?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let schema = Rc::new(Schema::from_binary(&buf)?);

    Config::from_schema(Rc::downgrade(&schema))?;
    Ok(())
}
