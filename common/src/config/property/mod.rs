mod constraints;

use crate::context::Context;
use crate::schema::Schema;
use constraints::Constraints;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Property {
    pub key: String,
    pub path: String,
    values: RefCell<Vec<String>>,
    constraints: Constraints,
}

#[derive(Debug)]
pub enum PropertyError {
    DefaultResolvingError { source: Box<dyn std::error::Error> },
    ConstraintNotMetError,
}

impl std::fmt::Display for PropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            PropertyError::DefaultResolvingError { source: _ } => {
                write!(f, "default value failed to resolve")
            }
            PropertyError::ConstraintNotMetError => write!(f, "constraint not met"),
        }
    }
}

impl std::error::Error for PropertyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            PropertyError::DefaultResolvingError { source } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl Property {
    pub fn from_schema_property(
        parent: &String,
        context: &Context,
        property: &crate::schema::property::Property,
    ) -> Result<Property, PropertyError> {
        let mut values = Vec::new();

        for default in &property.default {
            values.extend(match default.resolve(context) {
                Ok(v) => v,
                Err(e) => return Err(PropertyError::DefaultResolvingError { source: e }),
            });
        }

        if !property.multiple && values.len() > 1 {
            Err(PropertyError::ConstraintNotMetError)
        } else {
            Ok(Property {
                key: property.key.clone(),
                path: parent.clone(),
                values: RefCell::new(values),
                constraints: Constraints::from_schema_property(property),
            })
        }
    }

    pub fn values(&self) -> Vec<String> {
        self.values.borrow().iter().map(|v| v.to_owned()).collect()
    }

    pub fn set(&self, value: String, schema: &Schema) -> Result<(), PropertyError> {
        match self.constraints.matches(&value, schema) {
            Ok(()) => (),
            Err(e) => return Err(PropertyError::ConstraintNotMetError),
        }

        let mut values = self.values.borrow_mut();
        if !self.constraints.multiple {
            values.clear();
        }
        values.push(value);

        Ok(())
    }
}
