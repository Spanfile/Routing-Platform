mod constraints;

use super::Changeable;
use crate::error::PropertyError;
use anyhow::anyhow;
use colored::Colorize;
use constraints::Constraints;
use rp_common::Context;
use rp_schema::Schema;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyChange {
    Unchanged,
    New,
    Removed,
    Edited { old_value: String },
}

#[derive(Debug)]
pub struct Property {
    pub key: String,
    values: RefCell<HashMap<String, PropertyChange>>,
    // TODO: this is pretty horrible just look it up from the schema or smth
    default_values: Vec<String>,
    constraints: Constraints,
}

impl Property {
    pub fn from_schema_property(
        context: Rc<Context>,
        key: &str,
        property: &rp_schema::Property,
        schema: Weak<Schema>,
    ) -> anyhow::Result<Property> {
        let mut values = HashMap::new();
        let constraints = Constraints::from_schema_property(property);

        if let Some(schema) = schema.upgrade() {
            for default in &property.default {
                for v in default.resolve(&context)? {
                    constraints.matches(&v, schema.as_ref())?;
                    values.insert(v, PropertyChange::Unchanged);
                }
            }

            if !property.multiple && values.len() > 1 {
                Err(PropertyError::ConstraintNotMet.into())
            } else {
                Ok(Property {
                    key: key.to_owned(),
                    default_values: values.iter().map(|(value, _)| value.to_owned()).collect(),
                    values: RefCell::new(values),
                    constraints,
                })
            }
        } else {
            Err(anyhow!("schema weak pointer upgrade failed"))
        }
    }

    pub fn values(&self) -> Vec<String> {
        self.values
            .borrow()
            .iter()
            .map(|(value, _)| value.to_owned())
            .collect()
    }

    pub fn set(&self, value: &str, schema: &rp_schema::Schema) -> anyhow::Result<()> {
        self.constraints.matches(&value, schema)?;

        let mut values = self.values.try_borrow_mut()?;

        if !self.constraints.multiple {
            // multiple values aren't allowed so at this point there must be only one old
            // value
            if values.is_empty() {
                values.insert(value.to_string(), PropertyChange::New);
            } else {
                let old_value = values
                    .keys()
                    .nth(0)
                    .map(|k| k.clone())
                    .ok_or_else(|| anyhow!("values empty after check"))?;
                values.remove(&old_value);
                values.insert(value.to_string(), PropertyChange::Edited { old_value });
            }
        } else {
            values.insert(value.to_string(), PropertyChange::New);
        }

        Ok(())
    }

    pub fn remove(&self, value: Option<&str>) -> anyhow::Result<()> {
        if self.values.borrow().is_empty() {
            return Err(PropertyError::NoValueSet.into());
        }

        let mut values = self.values.try_borrow_mut()?;

        if let Some(value) = value {
            *values
                .get_mut(value)
                .ok_or_else(|| PropertyError::NoSuchValue(value.to_string()))? =
                PropertyChange::Removed;
        } else {
            for change in values.values_mut() {
                *change = PropertyChange::Removed;
            }
        }

        // TODO: test this
        // if values.is_empty() && !self.constraints.deletable {
        //     *values = self.default_values.clone();
        // }

        Ok(())
    }
}

impl Changeable for Property {
    fn is_clean(&self) -> bool {
        self.values
            .borrow()
            .values()
            .all(|change| *change == PropertyChange::Unchanged)
    }

    fn apply_changes(&self) -> anyhow::Result<()> {
        let new_values: HashMap<String, PropertyChange> = self
            .values
            .try_borrow()?
            .iter()
            .filter_map(|(value, change)| match change {
                PropertyChange::New | PropertyChange::Edited { .. } => {
                    Some((value.clone(), PropertyChange::Unchanged))
                }
                PropertyChange::Removed => None,
                _ => Some((value.clone(), change.clone())),
            })
            .collect();

        self.values.replace(new_values);

        Ok(())
    }

    fn discard_changes(&self) {
        let new_values: HashMap<String, PropertyChange> = self
            .values
            .borrow()
            .iter()
            .filter_map(|(value, change)| match change {
                PropertyChange::New => None,
                PropertyChange::Removed => Some((value.clone(), PropertyChange::Unchanged)),
                PropertyChange::Edited { old_value } => {
                    Some((old_value.clone(), PropertyChange::Unchanged))
                }
                _ => Some((value.clone(), change.clone())),
            })
            .collect();

        self.values.replace(new_values);
    }
}

impl Property {
    pub fn pretty_print(&self, indent: usize) {
        for (value, change) in self.values.borrow().iter() {
            match change {
                PropertyChange::New => println!(
                    "{:indent$}{}{} {}",
                    "",
                    "+".green(),
                    self.key.green(),
                    value.green(),
                    indent = indent * 4
                ),
                PropertyChange::Removed => println!(
                    "{:indent$}{}{} {}",
                    "",
                    "-".red(),
                    self.key.red(),
                    value.red(),
                    indent = indent * 4
                ),
                PropertyChange::Edited { old_value } => {
                    println!(
                        "{:indent$}{}{} {}",
                        "",
                        "-".red(),
                        self.key.red(),
                        old_value.red(),
                        indent = indent * 4
                    );
                    println!(
                        "{:indent$}{}{} {}",
                        "",
                        "-".red(),
                        self.key.red(),
                        value.red(),
                        indent = indent * 4
                    )
                }
                PropertyChange::Unchanged => {
                    println!("{:indent$}{} {}", "", self.key, value, indent = indent * 4);
                }
            }
        }
    }
}
