mod constraints;

use super::ConfigNode;
use crate::{context::Context, schema::Schema};
use constraints::Constraints;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Property {
    pub key: String,
    pub parent: Weak<ConfigNode>,
    values: RefCell<Vec<String>>,
    default_values: Vec<String>, // this is pretty horrible just look it up from the schema or smth
    constraints: Constraints,
}

#[derive(Debug)]
pub enum PropertyError {
    DefaultResolvingError { source: Box<dyn std::error::Error> },
    ConstraintNotMet,
    NoValueSet,
}

impl std::fmt::Display for PropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            PropertyError::DefaultResolvingError { .. } => {
                write!(f, "default value failed to resolve")
            }
            PropertyError::ConstraintNotMet => write!(f, "constraint not met"),
            PropertyError::NoValueSet => write!(f, "no value set"),
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
        parent: Weak<ConfigNode>,
        context: Rc<Context>,
        key: &str,
        property: &crate::schema::Property,
    ) -> Result<Property, PropertyError> {
        let mut values = Vec::new();

        for default in &property.default {
            values.extend(match default.resolve(&context) {
                Ok(v) => v,
                Err(e) => return Err(PropertyError::DefaultResolvingError { source: e }),
            });
        }

        if !property.multiple && values.len() > 1 {
            Err(PropertyError::ConstraintNotMet)
        } else {
            Ok(Property {
                key: key.to_owned(),
                parent,
                default_values: values.iter().map(|s| s.to_owned()).collect(),
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
            Err(_e) => return Err(PropertyError::ConstraintNotMet),
        }

        let mut values = self.values.borrow_mut();
        if !self.constraints.multiple {
            values.clear();
        }
        values.push(value);

        Ok(())
    }

    pub fn remove(&self, value: Option<String>) -> Result<(), PropertyError> {
        if self.values.borrow().is_empty() {
            return Err(PropertyError::NoValueSet);
        }

        let mut values = self.values.borrow_mut();
        if let Some(value) = value {
            values.retain(|v| *v != value);
        } else {
            values.clear();
        }

        if values.is_empty() && !self.constraints.deletable {
            *values = self.default_values.clone();
        }

        Ok(())
    }
}

impl Property {
    pub fn pretty_print(&self, indent: usize) {
        for value in self.values.borrow().iter() {
            println!("{:indent$}{} {}", "", self.key, value, indent = indent * 4);
        }
    }
}
