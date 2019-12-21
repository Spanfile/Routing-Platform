mod common;

use anyhow::anyhow;
use rp_shell::ConfigEditor;

fn assert_property(
    editor: &ConfigEditor,
    property: &str,
    expected: Option<&str>,
) -> anyhow::Result<()> {
    if let Some(values) = editor.get_property_values(Some(String::from(property))) {
        if let Some(values) = values.get(property) {
            if let Some(expected) = expected {
                if !values.contains(&String::from(expected)) {
                    Err(anyhow!("set value not in property values: {:?}", values))
                } else {
                    Ok(())
                }
            } else {
                if !values.is_empty() {
                    Err(anyhow!("values not empty: {:?}", values))
                } else {
                    Ok(())
                }
            }
        } else {
            Err(anyhow!(
                "get_property_values returned without the property: {:?}",
                values
            ))
        }
    } else {
        Err(anyhow!("get_property_values returned None"))
    }
}

#[test]
fn set_new_property_value() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("singlenode")?;
    editor.set_property_value("simple", "1")?;

    if editor.is_clean() {
        Err(anyhow!("config clean after change"))
    } else {
        editor.apply_changes()?;
        assert_property(&editor, "simple", Some("1"))
    }
}

#[test]
fn discard_changes() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("singlenode")?;
    editor.set_property_value("default", "1")?;

    if editor.is_clean() {
        Err(anyhow!("config clean after change"))
    } else {
        editor.discard_changes();

        if !editor.is_clean() {
            Err(anyhow!("configuration dirty after discard"))
        } else {
            assert_property(&editor, "default", Some("0"))
        }
    }
}

#[test]
fn edit_existing_property_value() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("singlenode")?;
    editor.set_property_value("default", "1")?;

    if editor.is_clean() {
        Err(anyhow!("config clean after change"))
    } else {
        editor.apply_changes()?;
        assert_property(&editor, "default", Some("1"))
    }
}

#[test]
fn remove_existing_property_value() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("singlenode")?;
    editor.remove_property_value("multiple", Some("0"))?;

    if editor.is_clean() {
        Err(anyhow!("config clean after change"))
    } else {
        editor.apply_changes()?;
        assert_property(&editor, "multiple", Some("1"))
    }
}

#[test]
fn remove_all_property_values() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("singlenode")?;
    editor.remove_property_value("query_default", None)?;

    if editor.is_clean() {
        Err(anyhow!("config clean after change"))
    } else {
        editor.apply_changes()?;
        assert_property(&editor, "query_default", None)
    }
}

#[test]
fn remove_nonexistent_property_value() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("singlenode")?;

    let result = editor.remove_property_value("query_default", Some("nonexistent"));

    match result {
        Ok(_) => Err(anyhow!("nonexistent value removal succeeded")),
        Err(e) => {
            if let Some(rp_core::error::PropertyError::NoSuchValue { .. }) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn remove_node() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("multinode")?;
    editor.edit_node("0")?;
    editor.go_up()?;
    editor.remove_node("0")?;

    if editor.is_clean() {
        Err(anyhow!("config clean after change"))
    } else {
        editor.apply_changes()?;

        if editor.get_available_nodes().contains(&String::from("0")) {
            Err(anyhow!("node not removed after removal"))
        } else {
            Ok(())
        }
    }
}

#[test]
fn remove_nonexistent_node() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("multinode")?;
    let result = editor.remove_node("nonexistent");

    match result {
        Ok(_) => Err(anyhow!("nonexistent node removal succeeded")),
        Err(e) => {
            if let Some(rp_core::error::NodeRemovalError { .. }) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn remove_top_level_node() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let editor = ConfigEditor::new(&config, schema.as_ref());

    let result = editor.remove_node("singlenode");

    match result {
        Ok(_) => Err(anyhow!("top-level node removal succeeded")),
        Err(e) => {
            if let Some(rp_core::error::NodeRemovalError { .. }) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}
