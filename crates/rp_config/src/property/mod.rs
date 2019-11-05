mod constraints;

use crate::error::PropertyError;
use anyhow::anyhow;
use constraints::Constraints;
use rp_common::Context;
use rp_schema::Schema;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

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
        property: &rp_schema::Property,
        schema: Weak<Schema>,
    ) -> anyhow::Result<Property> {
        let mut values = Vec::new();
        let constraints = Constraints::from_schema_property(property);

        if let Some(schema) = schema.upgrade() {
            for default in &property.default {
                for v in default.resolve(&context)? {
                    constraints.matches(&v, schema.as_ref())?;
                    values.push(v);
                }
            }

            if !property.multiple && values.len() > 1 {
                Err(PropertyError::ConstraintNotMet.into())
            } else {
                Ok(Property {
                    key: key.to_owned(),
                    default_values: values.iter().map(|s| s.to_owned()).collect(),
                    values: RefCell::new(values),
                    constraints,
                })
            }
        } else {
            Err(anyhow!("schema weak pointer upgrade failed"))
        }
    }

    pub fn values(&self) -> Vec<String> {
        self.values.borrow().iter().map(|v| v.to_owned()).collect()
    }

    pub fn set(&self, value: &str, schema: &rp_schema::Schema) -> anyhow::Result<()> {
        self.constraints.matches(&value, schema)?;

        let mut values = self.values.borrow_mut();
        if !self.constraints.multiple {
            values.clear();
        }
        values.push(value.to_string());

        Ok(())
    }

    pub fn remove(&self, value: Option<&str>) -> anyhow::Result<()> {
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
