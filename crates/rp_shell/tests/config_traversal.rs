mod common;

use anyhow::anyhow;
use rp_shell::ConfigEditor;

#[test]
fn traversal() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("singlenode")?;
    editor.edit_node("subnode")?;

    if editor.get_current_node() == Some(String::from("subnode")) {
        Ok(())
    } else {
        Err(anyhow!("current node not 'subnode'"))
    }
}

#[test]
fn traversal_invalid() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("singlenode")?;
    let result = editor.edit_node("nonexistent");

    match result {
        Ok(_) => Err(anyhow!("singlenode traversal succeeded")),
        Err(e) => {
            if let Some(rp_shell::error::ConfigEditorError::NodeNotFound { .. }) = e.downcast_ref()
            {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn traversal_up() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("singlenode")?;
    editor.edit_node("subnode")?;

    editor.go_up()?;

    if editor.get_current_node() == Some(String::from("singlenode")) {
        Ok(())
    } else {
        Err(anyhow!("current node not 'singlenode'"))
    }
}

#[test]
fn traversal_up_at_root() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    let result = editor.go_up();

    match result {
        Ok(_) => Err(anyhow!("going up succeeded")),
        Err(e) => {
            if let Some(rp_shell::error::ConfigEditorError::AlreadyAtTop) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn traversal_top() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("singlenode")?;
    editor.edit_node("subnode")?;

    editor.go_top()?;

    if editor.get_current_node() == None {
        Ok(())
    } else {
        Err(anyhow!("current node not top node (None)'"))
    }
}

#[test]
fn traversal_top_at_root() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    let result = editor.go_top();

    match result {
        Ok(_) => Err(anyhow!("going to top succeeded")),
        Err(e) => {
            if let Some(rp_shell::error::ConfigEditorError::AlreadyAtTop) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn traverse_multinode() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("multinode")?;
    editor.edit_node("0")?;

    if editor.get_current_node() == Some(String::from("0")) {
        Ok(())
    } else {
        Err(anyhow!("current node not '0'"))
    }
}

#[test]
fn traverse_multinode_invalid() -> anyhow::Result<()> {
    let (schema, config) = common::get_schema_and_config()?;
    let mut editor = ConfigEditor::new(&config, schema.as_ref());

    editor.edit_node("multinode")?;
    let result = editor.edit_node("text");

    match result {
        Ok(_) => Err(anyhow!("multinode traversal succeeded")),
        Err(e) => {
            if let Some(rp_shell::error::ConfigEditorError::NodeNotFound { .. }) = e.downcast_ref()
            {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}
