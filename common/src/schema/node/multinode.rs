use super::{Node, NodeLocator, Schema, Validate, ValidationError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Multinode {
    pub template: String,
    pub node: Box<Node>,
}

impl Multinode {
    pub fn get_node_locator(&self) -> NodeLocator {
        self.node.get_locator()
    }
}

impl Validate for Multinode {
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
