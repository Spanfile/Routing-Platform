mod common;
mod property;
mod template;

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
    let mut schema = common::get_invalid_regex_template_schema()?;
    if schema.build_regex_cache().is_err() {
        Ok(())
    } else {
        Err(anyhow!("invalid regex template serialised succesfully"))
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

#[test]
fn merge_template_without_conflict() -> anyhow::Result<()> {
    let mut schema = common::get_valid_schema()?;
    let new_schema = common::get_new_template_schema()?;

    schema.merge(new_schema, MergingStrategy::Error)?;
    if let Template::Regex(_template) = schema
        .templates
        .get(&String::from("new"))
        .ok_or_else(|| anyhow!("schema doesn't have new template"))?
        .as_ref()
    {
        Ok(())
    } else {
        Err(anyhow!("schema new template isn't a regex template"))
    }
}

#[test]
fn merge_ours_template() -> anyhow::Result<()> {
    let mut schema = common::get_valid_schema()?;
    let new_schema = common::get_merge_template_schema()?;

    schema.merge(new_schema, MergingStrategy::Ours)?;
    if let Template::Regex(template) = schema
        .templates
        .get(&String::from("string"))
        .ok_or_else(|| anyhow!("schema doesn't have string template"))?
        .as_ref()
    {
        if template.regex != ".*" {
            Err(anyhow!(
                "string template regex isn't .* (is {} instead)",
                template.regex
            ))
        } else {
            Ok(())
        }
    } else {
        Err(anyhow!("schema string template isn't a regex template"))
    }
}

#[test]
fn merge_theirs_template() -> anyhow::Result<()> {
    let mut schema = common::get_valid_schema()?;
    let new_schema = common::get_merge_template_schema()?;

    schema.merge(new_schema, MergingStrategy::Theirs)?;
    if let Template::Regex(template) = schema
        .templates
        .get(&String::from("string"))
        .ok_or_else(|| anyhow!("schema doesn't have string template"))?
        .as_ref()
    {
        if template.regex != "a" {
            Err(anyhow!(
                "string template regex isn't a (is {} instead)",
                template.regex
            ))
        } else {
            Ok(())
        }
    } else {
        Err(anyhow!("schema string template isn't a regex template"))
    }
}

#[test]
fn merge_ours_node() -> anyhow::Result<()> {
    let mut schema = common::get_valid_schema()?;
    let new_schema = common::get_merge_node_schema()?;

    schema.merge(new_schema, MergingStrategy::Ours)?;
    let locator = Rc::new(NodeLocator::new(
        String::from("system"),
        Some(Rc::new(NodeLocator::new(String::from("schema"), None))),
    ));

    if let Some(node) = schema.find_node(locator) {
        if let SchemaNode::SingleSchemaNode(node) = node {
            if let Some(property) = node.properties.get(&String::from("hostname")) {
                if property.deletable != false {
                    Err(anyhow!("hostname property is deletable"))
                } else {
                    Ok(())
                }
            } else {
                Err(anyhow!("hostname property not in system node"))
            }
        } else {
            Err(anyhow!("system node not a SingleSchemaNode"))
        }
    } else {
        Err(anyhow!("system node not in schema"))
    }
}

#[test]
fn merge_theirs_node() -> anyhow::Result<()> {
    let mut schema = common::get_valid_schema()?;
    let new_schema = common::get_merge_node_schema()?;

    schema.merge(new_schema, MergingStrategy::Theirs)?;
    let locator = Rc::new(NodeLocator::new(
        String::from("system"),
        Some(Rc::new(NodeLocator::new(String::from("schema"), None))),
    ));

    if let Some(node) = schema.find_node(locator) {
        if let SchemaNode::SingleSchemaNode(node) = node {
            if let Some(property) = node.properties.get(&String::from("hostname")) {
                if property.deletable != true {
                    Err(anyhow!("hostname property is not deletable"))
                } else {
                    Ok(())
                }
            } else {
                Err(anyhow!("hostname property not in system node"))
            }
        } else {
            Err(anyhow!("system node not a SingleSchemaNode"))
        }
    } else {
        Err(anyhow!("system node not in schema"))
    }
}

#[test]
fn merge_new_node() -> anyhow::Result<()> {
    let mut schema = common::get_valid_schema()?;
    let new_schema = common::get_new_node_schema()?;

    schema.merge(new_schema, MergingStrategy::Error)?;
    let locator = Rc::new(NodeLocator::new(
        String::from("interfaces"),
        Some(Rc::new(NodeLocator::new(String::from("schema"), None))),
    ));

    if let Some(node) = schema.find_node(locator) {
        if let SchemaNode::SingleSchemaNode(_node) = node {
            Ok(())
        } else {
            Err(anyhow!("interfaces node not a SingleSchemaNode"))
        }
    } else {
        Err(anyhow!("interfaces node not in schema"))
    }
}
