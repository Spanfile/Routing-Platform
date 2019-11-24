mod common;

use std::io::Cursor;

fn buffer() -> Cursor<Vec<u8>> {
    Cursor::new(Vec::new())
}

#[test]
fn valid_config() -> anyhow::Result<()> {
    common::get_valid_config()?;
    Ok(())
}

#[test]
fn save() -> anyhow::Result<()> {
    let config = common::get_valid_config()?;
    let buf = buffer();

    config.save_config(buf)?;
    Ok(())
}
