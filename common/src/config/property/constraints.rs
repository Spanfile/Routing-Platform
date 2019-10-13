use crate::{
    error,
    error::ConstraintError,
    schema::{Matches, Property, Schema, Value},
};

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
            values: property.values.to_vec(),
            deletable: property.deletable,
        }
    }

    pub fn matches(&self, value: &str, schema: &Schema) -> error::CommonResult<()> {
        for v in &self.values {
            match v {
                Value::Literal(literal) => {
                    if value == literal {
                        return Ok(());
                    }
                }
                Value::Template(template) => {
                    let schema_template = schema
                        .templates
                        .get(template)
                        .expect("value template not found in schema templates");
                    if schema_template.matches(value) {
                        return Ok(());
                    }
                }
                Value::Range(range) => {
                    if !range.matches(value) {
                        continue;
                    }

                    return Ok(());
                }
            }
        }

        Err(ConstraintError {
            given: value.to_string(),
            allowed_values: self.values.clone(),
            source: None,
        }
        .into())
    }
}
