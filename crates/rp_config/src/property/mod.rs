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

#[derive(Debug)]
pub enum PropertyChange {
    New,
    Removed,
    Edited { old_value: String },
}

#[derive(Debug)]
pub struct Property {
    pub key: String,
    values: RefCell<Vec<String>>,
    default_values: Vec<String>, // this is pretty horrible just look it up from the schema or smth
    constraints: Constraints,
    changes: RefCell<HashMap<String, PropertyChange>>,
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
                    changes: RefCell::new(HashMap::new()),
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

        let mut changes = self.changes.try_borrow_mut()?;
        if !self.constraints.multiple {
            let mut old_changed = false;

            for v in self.values.try_borrow()?.iter() {
                changes.insert(
                    v.to_owned(),
                    if v == value {
                        old_changed = true;
                        PropertyChange::Edited {
                            old_value: v.to_owned(),
                        }
                    } else {
                        PropertyChange::Removed
                    },
                );
            }

            if !old_changed {
                changes.insert(value.to_string(), PropertyChange::New);
            }
        } else {
            changes.insert(value.to_string(), PropertyChange::New);
        }

        Ok(())
    }

    pub fn remove(&self, value: Option<&str>) -> anyhow::Result<()> {
        if self.values.borrow().is_empty() {
            return Err(PropertyError::NoValueSet.into());
        }

        let mut changes = self.changes.try_borrow_mut()?;

        if let Some(value) = value {
            if !self.values.borrow().iter().any(|v| v == value) {
                return Err(PropertyError::NoSuchValue(value.to_string()).into());
            }

            changes.insert(value.to_owned(), PropertyChange::Removed);
        } else {
            for v in self.values.try_borrow()?.iter() {
                changes.insert(v.to_owned(), PropertyChange::Removed);
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
        self.changes.borrow().is_empty()
    }

    fn apply_changes(&self) -> anyhow::Result<()> {
        let mut values = self.values.try_borrow_mut()?;
        let mut changes = self.changes.try_borrow_mut()?;

        for (node, change) in changes.iter() {
            match change {
                PropertyChange::New => values.push(node.to_owned()),
                PropertyChange::Removed => {
                    values
                        .remove_item(node)
                        .ok_or_else(|| anyhow!("removed value not in values"))?;
                }
                PropertyChange::Edited { old_value } => {
                    values
                        .remove_item(old_value)
                        .ok_or_else(|| anyhow!("changed old value not in values"))?;
                    values.push(node.to_owned());
                }
            }
        }

        changes.clear();

        Ok(())
    }

    fn discard_changes(&self) {
        self.changes.borrow_mut().clear();
    }
}

impl Property {
    pub fn pretty_print(&self, indent: usize) {
        let changes = self.changes.borrow();

        for value in self.values.borrow().iter() {
            match changes.get(value) {
                Some(PropertyChange::New) => println!(
                    "{:indent$}{}{} {}",
                    "",
                    "+".green(),
                    self.key.green(),
                    value.green(),
                    indent = indent * 4
                ),
                Some(PropertyChange::Removed) => println!(
                    "{:indent$}{}{} {}",
                    "",
                    "-".red(),
                    self.key.red(),
                    value.red(),
                    indent = indent * 4
                ),
                Some(PropertyChange::Edited { old_value }) => {
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
                None => {
                    println!("{:indent$}{} {}", "", self.key, value, indent = indent * 4);
                }
            }
        }
    }
}
