mod common;

use super::*;
use anyhow::anyhow;

#[test]
fn schema_from_yaml() -> anyhow::Result<()> {
    common::get_valid_schema()?;
    Ok(())
}

#[test]
fn valid_schema() -> anyhow::Result<()> {
    let mut schema = common::get_valid_schema()?;
    schema.validate()
}

#[test]
fn valid_regex_cache() -> anyhow::Result<()> {
    let mut schema = common::get_valid_schema()?;
    schema.build_regex_cache()
}

#[test]
fn invalid_regex_cache() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_template_schema()?;
    if schema.build_regex_cache().is_err() {
        Ok(())
    } else {
        Err(anyhow!("invalid regex template serialised succesfully"))
    }
}

#[test]
fn invalid_template() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_template_schema()?;
    if schema.validate().is_err() {
        Ok(())
    } else {
        Err(anyhow!("invalid template validated succesfully"))
    }
}

#[test]
fn invalid_value() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_node_schema()?;
    if schema.validate().is_err() {
        Ok(())
    } else {
        Err(anyhow!("invalid value validated succesfully"))
    }
}

#[test]
fn find_existing_node() -> anyhow::Result<()> {
    let schema = common::get_valid_schema()?;
    let locator = Rc::new(NodeLocator::new(
        String::from("ntp"),
        Some(Rc::new(NodeLocator::new(
            String::from("system"),
            Some(Rc::new(NodeLocator::new(String::from("schema"), None))),
        ))),
    ));

    schema
        .find_node(locator)
        .ok_or_else(|| anyhow!("existing node not found"))?;
    Ok(())
}

#[test]
fn find_nonexistent_node() -> anyhow::Result<()> {
    let schema = common::get_valid_schema()?;
    let locator = Rc::new(NodeLocator::new(
        String::from("nonexistent"),
        Some(Rc::new(NodeLocator::new(
            String::from("system"),
            Some(Rc::new(NodeLocator::new(String::from("schema"), None))),
        ))),
    ));

    if schema.find_node(locator).is_none() {
        Ok(())
    } else {
        Err(anyhow!("nonexistent node found"))
    }
}

#[test]
fn node_count() -> anyhow::Result<()> {
    let schema = common::get_valid_schema()?;
    if schema.node_count() == 2 {
        Ok(())
    } else {
        Err(anyhow!("schema doesn't have exactly two nodes"))
    }
}

#[test]
fn property_count() -> anyhow::Result<()> {
    let schema = common::get_valid_schema()?;
    if schema.property_count() == 2 {
        Ok(())
    } else {
        Err(anyhow!("schema doesn't have exactly two properties"))
    }
}
