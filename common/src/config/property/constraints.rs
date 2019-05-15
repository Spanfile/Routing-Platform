use crate::schema::property::Property;
use crate::schema::value::{Bound, Value};
use crate::schema::Schema;

#[derive(Debug)]
pub struct Constraints {
    pub multiple: bool,
    values: Vec<Value>,
}

#[derive(Debug)]
pub struct ConstraintError {}

impl std::fmt::Display for ConstraintError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "constraints not met")
    }
}

impl std::error::Error for ConstraintError {}

impl Constraints {
    pub fn from_schema_property(property: &Property) -> Constraints {
        Constraints {
            multiple: property.multiple,
            values: property.values.to_vec(),
        }
    }

    pub fn matches(&self, value: &String, schema: &Schema) -> Result<(), ConstraintError> {
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
                Value::Range { lower, upper } => {
                    let numeric: f64 = match value.parse() {
                        Ok(v) => v,
                        Err(_e) => continue,
                    };

                    match lower {
                        Bound::Inclusive(bound) => {
                            if numeric < *bound {
                                continue;
                            }
                        }
                        Bound::Exclusive(bound) => {
                            if numeric <= *bound {
                                continue;
                            }
                        }
                    }

                    match upper {
                        Bound::Inclusive(bound) => {
                            if numeric > *bound {
                                continue;
                            }
                        }
                        Bound::Exclusive(bound) => {
                            if numeric >= *bound {
                                continue;
                            }
                        }
                    }

                    return Ok(());
                }
            }
        }

        Err(ConstraintError {})
    }
}
