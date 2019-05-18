pub mod range;

use super::query::Query;
use crate::context::Context;
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

#[derive(Serialize, Deserialize, Debug)]
pub enum DefaultValue {
    #[serde(rename = "literal")]
    Literal(String),
    #[serde(rename = "query")]
    Query(Query),
}

impl DefaultValue {
    pub fn resolve(&self, context: &Context) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        match self {
            DefaultValue::Literal(literal) => Ok(vec![literal.to_owned()]),
            DefaultValue::Query(query) => query.run(context),
        }
    }
}
