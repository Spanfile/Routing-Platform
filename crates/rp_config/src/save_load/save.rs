use crate::error::SaveError;
use rp_log::*;
use serde::Serialize;
use serde_json;
use std::{cell::RefCell, collections::HashMap, io::Write, rc::Rc};

pub fn save<T>(thing: &dyn Save, dest: T) -> anyhow::Result<()>
where
    T: Write,
{
    let mut builder = SaveBuilder::new();
    thing.save(&mut builder)?;
    trace!("Built save: {:?}", builder);
    serde_json::to_writer(dest, &builder)?;
    Ok(())
}

pub trait Save {
    fn save(&self, builder: &mut SaveBuilder) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize)]
pub struct SaveBuilder {
    nodes: HashMap<String, Rc<SaveNode>>,
    #[serde(skip)]
    node_stack: Vec<Rc<SaveNode>>,
}

impl SaveBuilder {
    fn new() -> Self {
        SaveBuilder {
            nodes: HashMap::new(),
            node_stack: Vec::new(),
        }
    }

    pub fn begin_node(&mut self, name: String) -> anyhow::Result<()> {
        let new_node = Rc::new(SaveNode::new());

        if let Some(node) = self.node_stack.last() {
            node.insert_node(name, Rc::clone(&new_node))?;
        } else {
            self.nodes.insert(name, Rc::clone(&new_node));
        }

        self.node_stack.push(new_node);
        Ok(())
    }

    pub fn end_node(&mut self) -> anyhow::Result<()> {
        self.node_stack.pop().ok_or(SaveError::NoNodeToEnd)?;
        Ok(())
    }

    pub fn set_property(&mut self, name: String, value: String) -> anyhow::Result<()> {
        if let Some(node) = self.node_stack.last() {
            node.set_property(name, value)?;
            Ok(())
        } else {
            Err(SaveError::NoNodeToSetProperty.into())
        }
    }
}

#[derive(Debug, Serialize)]
struct SaveNode {
    subnodes: RefCell<HashMap<String, Rc<SaveNode>>>,
    properties: RefCell<HashMap<String, Vec<String>>>,
}

impl SaveNode {
    pub fn new() -> Self {
        SaveNode {
            subnodes: RefCell::new(HashMap::new()),
            properties: RefCell::new(HashMap::new()),
        }
    }

    pub fn insert_node(&self, name: String, node: Rc<SaveNode>) -> anyhow::Result<()> {
        self.subnodes.try_borrow_mut()?.insert(name, node);
        Ok(())
    }

    pub fn set_property(&self, name: String, value: String) -> anyhow::Result<()> {
        self.properties
            .try_borrow_mut()?
            .entry(name)
            .or_insert(Vec::new())
            .push(value);
        Ok(())
    }
}
