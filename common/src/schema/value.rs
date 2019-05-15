use super::query::Query;
use crate::context::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    #[serde(rename = "literal")]
    Literal(String),
    #[serde(rename = "template")]
    Template(String),
    #[serde(rename = "range")]
    Range { lower: Bound, upper: Bound },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Bound {
    #[serde(rename = "inclusive")]
    Inclusive(f64),
    #[serde(rename = "exclusive")]
    Exclusive(f64),
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
