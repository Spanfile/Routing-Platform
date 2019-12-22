use super::{Schema, SourceCommand, Validate};
use crate::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiSchemaNodeSource {
    pub id: String,
    pub template: Option<String>,
    pub command: SourceCommand,
}

impl Validate for MultiSchemaNodeSource {
    fn validate(&self, schema: &Schema) -> anyhow::Result<()> {
        if let Some(template) = &self.template {
            if !schema.templates.contains_key(template) {
                return Err(error::SchemaValidationError::MissingTemplate(template.clone()).into());
            }
        }

        Ok(())
    }
}
