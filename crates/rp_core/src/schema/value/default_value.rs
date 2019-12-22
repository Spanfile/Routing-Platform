use super::SourceCommand;
use crate::common::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum DefaultValue {
    Literal(String),
    Command(SourceCommand),
}

impl DefaultValue {
    pub fn resolve(&self, context: &Context) -> anyhow::Result<Vec<String>> {
        match self {
            DefaultValue::Literal(literal) => Ok(vec![literal.to_owned()]),
            DefaultValue::Command(command) => unimplemented!(),
        }
    }
}
