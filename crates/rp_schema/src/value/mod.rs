mod default_value;
pub mod range;

pub use default_value::DefaultValue;
use range::Range;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
