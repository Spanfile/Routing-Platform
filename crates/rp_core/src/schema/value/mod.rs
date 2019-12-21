mod default_value;
pub mod range;

use super::{Schema, Validate};
pub use default_value::DefaultValue;
use range::Range;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    #[serde(rename = "literal")]
    Literal(String),
    #[serde(rename = "template")]
    Template(String),
    #[serde(rename = "range")]
    Range(Range),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Literal(value) => write!(f, "'{}'", value),
            Value::Template(template) => write!(f, "template '{}'", template),
            Value::Range(range) => write!(f, "{}", range),
        }
    }
}

impl Validate for Value {
    fn validate(&self, schema: &Schema) -> anyhow::Result<()> {
        match self {
            Self::Template(template) => {
                if !schema.templates.contains_key(template) {
                    Err(
                        crate::error::SchemaValidationError::MissingTemplate(template.to_owned())
                            .into(),
                    )
                } else {
                    Ok(())
                }
            }
            Self::Range(range) => range.validate(schema),
            _ => Ok(()),
        }
    }
}
