use super::{NodeLocator, Schema, SingleSchemaNode, Validate, ValidationError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MultiSchemaNode {
    pub template: String,
    pub node: SingleSchemaNode,
}

impl MultiSchemaNode {
    pub fn node_count(&self) -> usize {
        1
    }

    pub fn property_count(&self) -> usize {
        0
    }

    pub fn get_locator(&self) -> NodeLocator {
        self.node.get_locator()
    }
}

impl Validate for MultiSchemaNode {
    fn validate(&self, schema: &Schema) -> Vec<ValidationError> {
        let mut errors: Vec<ValidationError> = Vec::new();
        errors.extend(self.node.validate(schema));

        if !schema.templates.contains_key(&self.template) {
            errors.push(ValidationError::new(format!(
                "Multinode validation error\nTemplate '{}' doesn't exist",
                self.template
            )));
        }

        errors
    }
}
