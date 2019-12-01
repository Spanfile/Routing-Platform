use crate::error::LoadError;
use chrono::prelude::*;
use rp_log::*;
use serde::Deserialize;
use std::{collections::HashMap, io::Read, rc::Rc};

pub fn load<T>(thing: &dyn Load, src: T) -> anyhow::Result<()>
where
    T: Read,
{
    let mut source = serde_json::from_reader::<T, LoadSource>(src)?;
    trace!("Loading from source: {:?}", source);
    thing.load(&mut source)
}

pub trait Load {
    fn load(&self, source: &mut LoadSource) -> anyhow::Result<()>;
}

#[derive(Debug, Deserialize)]
pub struct LoadSource {
    timestamp: DateTime<Utc>,
    nodes: HashMap<String, Rc<LoadNode>>,
    #[serde(skip)]
    node_stack: Vec<Rc<LoadNode>>,
}

impl LoadSource {
    pub fn begin_node(&mut self, name: &str) -> anyhow::Result<()> {
        let new_node = if let Some(node) = self.node_stack.last() {
            node.get_node(name)
        } else {
            self.nodes
                .get(name)
                .map(|n| Rc::clone(n))
                .ok_or_else(|| LoadError::NoSuchNode(name.to_owned()).into())
        }?;

        self.node_stack.push(new_node);
        Ok(())
    }

    pub fn end_node(&mut self) -> anyhow::Result<()> {
        self.node_stack.pop().ok_or(LoadError::NoNodeToEnd)?;
        Ok(())
    }

    pub fn get_property(&self, name: &str) -> anyhow::Result<&Vec<String>> {
        if let Some(node) = self.node_stack.last() {
            node.get_property(name)
        } else {
            Err(LoadError::NoNodeToGetProperty(name.to_owned()).into())
        }
    }

    pub fn get_node_names(&self) -> Vec<String> {
        if let Some(node) = self.node_stack.last() {
            node.get_node_names()
        } else {
            self.nodes.keys().map(|s| s.to_owned()).collect()
        }
    }
}

#[derive(Debug, Deserialize)]
struct LoadNode {
    subnodes: HashMap<String, Rc<LoadNode>>,
    properties: HashMap<String, Vec<String>>,
}

impl LoadNode {
    pub fn get_node(&self, name: &str) -> anyhow::Result<Rc<LoadNode>> {
        self.subnodes
            .get(name)
            .map(|n| Rc::clone(n))
            .ok_or_else(|| LoadError::NoSuchNode(name.to_string()).into())
    }

    pub fn get_property(&self, name: &str) -> anyhow::Result<&Vec<String>> {
        self.properties
            .get(name)
            .ok_or_else(|| LoadError::NoSuchProperty(name.to_owned()).into())
    }

    pub fn get_node_names(&self) -> Vec<String> {
        self.subnodes.keys().map(|s| s.to_owned()).collect()
    }
}
