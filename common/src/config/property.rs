use crate::context::Context;
use crate::schema::value::Value;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Property {
    pub key: String,
    pub path: String,
    pub values: RefCell<Vec<String>>,
    pub constraints: Constraints,
}

#[derive(Debug)]
pub struct Constraints {
    pub multiple: bool,
    pub values: Vec<Value>,
}

#[derive(Debug)]
pub enum PropertyError {
    DefaultResolvingError { cause: Box<dyn std::error::Error> },
    SchemaConstraintError,
}

impl std::fmt::Display for PropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            PropertyError::DefaultResolvingError { cause: _ } => {
                write!(f, "default value failed to resolve")
            }
            PropertyError::SchemaConstraintError => {
                write!(f, "default values break schema constraint")
            }
        }
    }
}

impl std::error::Error for PropertyError {
    fn cause(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            PropertyError::DefaultResolvingError { cause } => Some(cause.as_ref()),
            PropertyError::SchemaConstraintError => None,
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
                Err(e) => return Err(PropertyError::DefaultResolvingError { cause: e }),
            });
        }

        if !property.multiple && values.len() > 1 {
            Err(PropertyError::SchemaConstraintError)
        } else {
            Ok(Property {
                key: property.key.clone(),
                path: parent.clone(),
                values: RefCell::new(values),
                constraints: Constraints {
                    multiple: property.multiple,
                    values: property.values.to_vec(),
                },
            })
        }
    }
}
