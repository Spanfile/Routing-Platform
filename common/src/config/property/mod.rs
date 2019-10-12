mod constraints;

use crate::{context::Context, error, error::PropertyError, schema::Schema};
use constraints::Constraints;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Property {
    pub key: String,
    values: RefCell<Vec<String>>,
    default_values: Vec<String>, // this is pretty horrible just look it up from the schema or smth
    constraints: Constraints,
}

impl Property {
    pub fn from_schema_property(
        context: Rc<Context>,
        key: &str,
        property: &crate::schema::Property,
    ) -> error::CommonResult<Property> {
        let mut values = Vec::new();

        for default in &property.default {
            values.extend(match default.resolve(&context) {
                Ok(v) => v,
                Err(e) => return Err(PropertyError::DefaultResolvingError.into()),
            });
        }

        if !property.multiple && values.len() > 1 {
            Err(PropertyError::ConstraintNotMet.into())
        } else {
            Ok(Property {
                key: key.to_owned(),
                default_values: values.iter().map(|s| s.to_owned()).collect(),
                values: RefCell::new(values),
                constraints: Constraints::from_schema_property(property),
            })
        }
    }

    pub fn values(&self) -> Vec<String> {
        self.values.borrow().iter().map(|v| v.to_owned()).collect()
    }

    pub fn set(&self, value: String, schema: &Schema) -> error::CommonResult<()> {
        match self.constraints.matches(&value, schema) {
            Ok(()) => (),
            Err(_e) => return Err(PropertyError::ConstraintNotMet.into()),
        }

        let mut values = self.values.borrow_mut();
        if !self.constraints.multiple {
            values.clear();
        }
        values.push(value);

        Ok(())
    }

    pub fn remove(&self, value: Option<String>) -> error::CommonResult<()> {
        if self.values.borrow().is_empty() {
            return Err(PropertyError::NoValueSet.into());
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
