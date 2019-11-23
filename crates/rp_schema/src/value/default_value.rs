use crate::Query;
use rp_common::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefaultValue {
    #[serde(rename = "literal")]
    Literal(String),
    #[serde(rename = "query")]
    Query(Query),
}

impl DefaultValue {
    pub fn resolve(&self, context: &Context) -> anyhow::Result<Vec<String>> {
        match self {
            DefaultValue::Literal(literal) => Ok(vec![literal.to_owned()]),
            DefaultValue::Query(query) => query.run(context),
        }
    }
}
