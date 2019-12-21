use super::*;
use anyhow::anyhow;

#[test]
fn invalid_regex_template() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_regex_template_schema()?;
    let result = schema.validate();

    match result {
        Ok(_) => Err(anyhow!("template validation succeeded")),
        Err(e) => {
            if let Some(crate::error::SchemaValidationError::Regex { .. }) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn invalid_incl_incl_range_template() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_incl_incl_range_template_schema()?;
    let result = schema.validate();

    match result {
        Ok(_) => Err(anyhow!("template validation succeeded")),
        Err(e) => {
            if let Some(crate::error::SchemaValidationError::Range { .. }) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn invalid_excl_incl_range_template() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_excl_incl_range_template_schema()?;
    let result = schema.validate();

    match result {
        Ok(_) => Err(anyhow!("template validation succeeded")),
        Err(e) => {
            if let Some(crate::error::SchemaValidationError::Range { .. }) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn invalid_incl_excl_range_template() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_incl_excl_range_template_schema()?;
    let result = schema.validate();

    match result {
        Ok(_) => Err(anyhow!("template validation succeeded")),
        Err(e) => {
            if let Some(crate::error::SchemaValidationError::Range { .. }) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn invalid_excl_excl_range_template() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_excl_excl_range_template_schema()?;
    let result = schema.validate();

    match result {
        Ok(_) => Err(anyhow!("template validation succeeded")),
        Err(e) => {
            if let Some(crate::error::SchemaValidationError::Range { .. }) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}
