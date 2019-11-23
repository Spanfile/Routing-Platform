use super::{
    value::{DefaultValue, Value},
    Matches, Merge, MergingStrategy, Schema, Validate,
};
use crate::error;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Property {
    #[serde(default)]
    pub multiple: bool,
    #[serde(default)]
    pub default: HashSet<DefaultValue>,
    pub values: HashSet<Value>,
    #[serde(default = "Property::default_deletable")]
    pub deletable: bool,
}

impl Property {
    fn default_deletable() -> bool {
        true
    }
}

impl Validate for Property {
    fn validate(&self, schema: &Schema) -> anyhow::Result<()> {
        if self.values.is_empty() {
            return Err(error::SchemaValidationError::NoValues.into());
        }

        for value in &self.values {
            value.validate(schema)?;
        }

        if !self.default.is_empty() {
            if !self.multiple && self.default.len() > 1 {
                return Err(error::SchemaValidationError::NoMultipleValuesAllowed.into());
            }

            let mut match_found = false;
            for default in &self.default {
                match default {
                    DefaultValue::Literal(def) => {
                        for value in &self.values {
                            match value {
                                Value::Literal(literal) => {
                                    if def == literal {
                                        match_found = true;
                                        break;
                                    }
                                }
                                Value::Template(template) => {
                                    let templ =
                                        schema.templates.get(template).ok_or_else(|| {
                                            anyhow!(
                                                "Template {} not found after existence validation",
                                                template
                                            )
                                        })?;

                                    if templ.matches(def)? {
                                        match_found = true;
                                        break;
                                    }
                                }
                                Value::Range(range) => {
                                    if range.matches(def)? {
                                        match_found = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    _ => match_found = true,
                }

                if !match_found {
                    return Err(
                        error::SchemaValidationError::InvalidDefaultValue(default.clone()).into(),
                    );
                }
            }
        }

        Ok(())
    }
}

impl Merge for Property {
    fn merge(&mut self, other: Self, strategy: MergingStrategy) -> anyhow::Result<()> {
        self.multiple = strategy.resolve(self.multiple, other.multiple)?;
        self.deletable = strategy.resolve(self.deletable, other.deletable)?;

        for default in other.default {}

        Ok(())
    }
}
