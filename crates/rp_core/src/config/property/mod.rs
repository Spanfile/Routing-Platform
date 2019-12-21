mod constraints;

use super::Changeable;
use crate::{error::PropertyError, schema::Schema};
use anyhow::anyhow;
use colored::Colorize;
use constraints::Constraints;
use crate::common::Context;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

#[derive(Debug, PartialEq, Clone)]
enum PropertyChange {
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
    schema: Weak<Schema>,
}

impl Property {
    pub fn from_schema_property(
        context: Rc<Context>,
        key: &str,
        property: &crate::schema::Property,
        schema: Weak<Schema>,
    ) -> anyhow::Result<Property> {
        let mut values = HashMap::new();
        let constraints = Constraints::from_schema_property(property);

        if let Some(schema_rc) = schema.upgrade() {
            for default in &property.default {
                for v in default.resolve(&context)? {
                    constraints.matches(&v, schema_rc.as_ref())?;
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
                    schema,
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

    pub fn set(&self, value: &str) -> anyhow::Result<()> {
        let schema = self
            .schema
            .upgrade()
            .ok_or_else(|| anyhow!("schema weak pointer upgrade failed"))?;
        self.constraints.matches(&value, schema.as_ref())?;

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
                    .cloned()
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
        let mut match_made = false;
        let values: HashMap<String, PropertyChange> = self
            .values
            .try_borrow()?
            .iter()
            .filter_map(|(existing, change)| {
                if let Some(value) = value {
                    if value == existing {
                        match_made = true;
                    } else {
                        return Some((existing.clone(), change.clone()));
                    }
                }

                match change {
                    PropertyChange::New => None,
                    PropertyChange::Edited { old_value } => {
                        Some((old_value.clone(), PropertyChange::Removed))
                    }
                    PropertyChange::Unchanged | PropertyChange::Removed => {
                        Some((existing.clone(), PropertyChange::Removed))
                    }
                }
            })
            .collect();

        if let Some(value) = value {
            if !match_made {
                return Err(PropertyError::NoSuchValue(value.to_string()).into());
            }
        }

        if values.is_empty() {
            Err(PropertyError::NoValueSet.into())
        } else {
            self.values.replace(values);

            // TODO: test this
            // if values.is_empty() && !self.constraints.deletable {
            //     *values = self.default_values.clone();
            // }

            Ok(())
        }
    }
}

impl Changeable for Property {
    fn is_clean(&self) -> bool {
        self.values
            .borrow()
            .values()
            .all(|change| *change == PropertyChange::Unchanged)
    }

    fn apply_changes(&self) -> anyhow::Result<bool> {
        let mut edits = false;
        let new_values: HashMap<String, PropertyChange> = self
            .values
            .try_borrow()?
            .iter()
            .filter_map(|(value, change)| match change {
                PropertyChange::New | PropertyChange::Edited { .. } => {
                    edits = true;
                    Some((value.clone(), PropertyChange::Unchanged))
                }
                PropertyChange::Removed => {
                    edits = true;
                    None
                }
                PropertyChange::Unchanged => Some((value.clone(), change.clone())),
            })
            .collect();

        self.values.replace(new_values);

        Ok(edits)
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
                        "+".green(),
                        self.key.green(),
                        value.green(),
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
