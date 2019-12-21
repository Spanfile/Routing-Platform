mod common;

use anyhow::anyhow;
use rp_core::config::{Changeable, Node};
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
fn apply_changes() -> anyhow::Result<()> {
    let (config, _schema) = common::get_valid_config()?;
    let node = config
        .get_node_with_name("singlenode")
        .ok_or_else(|| anyhow!("'singlenode' node not in config"))?;
    let property = node
        .get_property("simple")
        .ok_or_else(|| anyhow!("'singlenode' node doesn't have property 'simple'"))?;

    property.set("a")?;

    if config.is_clean() {
        Err(anyhow!("config clean after change"))
    } else {
        if !config.apply_changes()? {
            Err(anyhow!("apply_changes reported no applied changes"))
        } else {
            if !config.is_clean() {
                Err(anyhow!("config dirty after applying changes"))
            } else {
                Ok(())
            }
        }
    }
}

#[test]
fn apply_no_changes() -> anyhow::Result<()> {
    let (config, _schema) = common::get_valid_config()?;

    if config.apply_changes()? {
        Err(anyhow!(
            "apply_changes reported applied changes on clean config"
        ))
    } else {
        Ok(())
    }
}

#[test]
fn save() -> anyhow::Result<()> {
    let (config, _schema) = common::get_valid_config()?;
    let buf = buffer();

    config.save_config(buf)?;
    Ok(())
}

#[test]
fn load() -> anyhow::Result<()> {
    let (config, _schema) = common::get_valid_config()?;
    let load_source = common::get_valid_save_data();
    config.load_config(load_source)?;

    Ok(())
}
