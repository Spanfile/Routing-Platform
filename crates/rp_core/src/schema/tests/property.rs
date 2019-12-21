use super::*;

#[test]
fn invalid_property_no_values() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_singlenode_prop_no_values_schema()?;
    let result = schema.validate();

    match result {
        Ok(_) => Err(anyhow!("property validation succeeded")),
        Err(e) => {
            if let Some(crate::error::SchemaValidationError::NoValues) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn invalid_property_value_template_missing() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_singlenode_prop_value_template_schema()?;
    let result = schema.validate();

    match result {
        Ok(_) => Err(anyhow!("property validation succeeded")),
        Err(e) => {
            if let Some(crate::error::SchemaValidationError::MissingTemplate(_)) = e.downcast_ref()
            {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn invalid_property_value_range() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_singlenode_prop_value_range_schema()?;
    let result = schema.validate();

    match result {
        Ok(_) => Err(anyhow!("property validation succeeded")),
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
fn invalid_property_multiple_default() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_singlenode_prop_multiple_defaults()?;
    let result = schema.validate();

    match result {
        Ok(_) => Err(anyhow!("property validation succeeded")),
        Err(e) => {
            if let Some(crate::error::SchemaValidationError::NoMultipleValuesAllowed) =
                e.downcast_ref()
            {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn invalid_property_invalid_default_literal() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_singlenode_prop_invalid_default_literal()?;
    let result = schema.validate();

    match result {
        Ok(_) => Err(anyhow!("property validation succeeded")),
        Err(e) => {
            if let Some(error::SchemaValidationError::InvalidDefaultValue(_)) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn invalid_property_invalid_default_template() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_singlenode_prop_invalid_default_template()?;
    let result = schema.validate();

    match result {
        Ok(_) => Err(anyhow!("property validation succeeded")),
        Err(e) => {
            if let Some(error::SchemaValidationError::InvalidDefaultValue(_)) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

#[test]
fn invalid_property_invalid_default_range() -> anyhow::Result<()> {
    let mut schema = common::get_invalid_singlenode_prop_invalid_default_range()?;
    let result = schema.validate();

    match result {
        Ok(_) => Err(anyhow!("property validation succeeded")),
        Err(e) => {
            if let Some(error::SchemaValidationError::InvalidDefaultValue(_)) = e.downcast_ref() {
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}
