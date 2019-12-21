#![feature(vec_remove_item)]

mod changeable;
pub mod error;
mod node;
mod node_name;
mod property;
mod save_load;

use anyhow::anyhow;
pub use changeable::Changeable;
pub use node::{ConfigNode, FromSchemaNode, Node};
pub use node_name::NodeName;
pub use property::Property;
use rp_common::Context;
use rp_log::*;
use rp_schema::Schema;
pub use save_load::{
    load::{load, Load, LoadSource},
    save::{save, Save, SaveBuilder},
};
use std::{
    cell::RefCell,
    collections::HashMap,
    io::{Read, Write},
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Config {
    pub nodes: HashMap<String, Rc<ConfigNode>>,
    unsaved: RefCell<bool>,
}

impl Config {
    pub fn from_schema(schema: Weak<Schema>) -> anyhow::Result<Config> {
        let mut nodes = HashMap::new();
        let mut context = Context::new(None);
        context.set_value(String::from("mock"), String::from("mock"));
        let context_rc = Rc::new(context);

        if let Some(s) = schema.upgrade() {
            for (name, node) in &s.nodes {
                nodes.insert(
                    name.to_owned(),
                    Rc::new(ConfigNode::from_schema_node(
                        Rc::clone(&context_rc),
                        &name,
                        Weak::clone(&schema),
                        &node,
                    )?),
                );
            }
        } else {
            return Err(anyhow!("Schema weak reference upgrading failed"));
        }

        Ok(Config {
            nodes,
            unsaved: RefCell::new(false),
        })
    }

    pub fn get_available_node_names(&self) -> Vec<NodeName> {
        let mut names = Vec::new();

        for name in self.nodes.keys() {
            names.push(NodeName::Literal(name.to_owned()));
        }

        names
    }

    pub fn get_node_with_name(&self, name: &str) -> Option<Rc<ConfigNode>> {
        self.nodes.get(name).map(|n| Rc::clone(n))
    }

    pub fn save_config<T>(&self, dest: T) -> anyhow::Result<()>
    where
        T: Write,
    {
        *self.unsaved.try_borrow_mut()? = false;
        save(self, dest)
    }

    pub fn has_unsaved_changes(&self) -> anyhow::Result<bool> {
        Ok(*self.unsaved.try_borrow()?)
    }

    pub fn load_config<T>(&self, src: T) -> anyhow::Result<()>
    where
        T: Read,
    {
        if let Err(e) = load(self, src) {
            trace!("Caught error while loading config; discarding changes",);
            self.discard_changes();
            Err(e)
        } else {
            if self.is_clean() {
                warn!(
                    "Loading new configuration didn't actually load anything (configuration clean)"
                );
            } else {
                self.apply_changes()?;
                *self.unsaved.try_borrow_mut()? = false;
            }

            Ok(())
        }
    }
}

impl Config {
    pub fn pretty_print(&self) {
        for (name, node) in &self.nodes {
            println!("{} {{", name);
            node.pretty_print(1);
            println!("}}");
        }
    }
}

impl Changeable for Config {
    fn is_clean(&self) -> bool {
        self.nodes.values().all(|node| node.is_clean())
    }

    fn apply_changes(&self) -> anyhow::Result<bool> {
        let mut edits = false;

        for node in self.nodes.values() {
            edits = node.apply_changes()? || edits;
        }

        if edits {
            *self.unsaved.try_borrow_mut()? = true;
        }

        Ok(edits)
    }

    fn discard_changes(&self) {
        for node in self.nodes.values() {
            node.discard_changes();
        }
    }
}

impl Save for Config {
    fn save(&self, builder: &mut SaveBuilder) -> anyhow::Result<()> {
        for (name, node) in &self.nodes {
            builder.begin_node(name.clone())?;
            node.save(builder)?;
            builder.end_node()?;
        }

        Ok(())
    }
}

impl Load for Config {
    fn load(&self, source: &mut LoadSource) -> anyhow::Result<()> {
        for (name, node) in &self.nodes {
            source.begin_node(name)?;
            node.load(source)?;
            source.end_node()?;
        }

        Ok(())
    }
}
