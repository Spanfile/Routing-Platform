use crate::{
    error::ConstraintError,
    schema::{Matches, Property, Schema, Value},
};
use anyhow::anyhow;

#[derive(Debug)]
pub struct Constraints {
    pub multiple: bool,
    values: Vec<Value>,
    pub deletable: bool,
}

impl Constraints {
    pub fn from_schema_property(property: &Property) -> Constraints {
        Constraints {
            multiple: property.multiple,
            values: property.values.clone().into_iter().collect(),
            deletable: property.deletable,
        }
    }

    pub fn matches(&self, value: &str, schema: &Schema) -> anyhow::Result<()> {
        for v in &self.values {
            match v {
                Value::Literal(literal) => {
                    if value == literal {
                        return Ok(());
                    }
                }
                Value::Template(template) => {
                    let schema_template = schema.templates.get(template).ok_or_else(|| {
                        anyhow!("Value template '{}' not found in schema templates", v)
                    })?;
                    if schema_template.matches(value)? {
                        return Ok(());
                    }
                }
                Value::Range(range) => {
                    if !range.matches(value)? {
                        continue;
                    }

                    return Ok(());
                }
            }
        }

        Err(ConstraintError {
            given: value.to_string(),
            allowed_values: self.values.clone(),
        }
        .into())
    }
}
