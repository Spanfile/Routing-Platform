use crate::error::LoadError;
use chrono::prelude::*;
use rp_log::*;
use serde::Deserialize;
use std::{collections::HashMap, io::Read};

pub fn load<T>(thing: &dyn Load, src: T) -> anyhow::Result<()>
where
    T: Read,
{
    let mut source = serde_json::from_reader::<T, LoadSource>(src)?;
    trace!("Loading from source timestamped {}", source.timestamp);
    thing.load(&mut source)
}

pub trait Load {
    fn load<'a>(&self, source: &'a mut LoadSource<'a>) -> anyhow::Result<()>;
}

#[derive(Debug, Deserialize)]
pub struct LoadSource<'a> {
    timestamp: DateTime<Utc>,
    nodes: HashMap<String, LoadNode>,
    #[serde(skip)]
    node_stack: Vec<&'a LoadNode>,
}

impl<'a> LoadSource<'a> {
    pub fn begin_node(&'a mut self, name: String) -> anyhow::Result<()> {
        let new_node = if let Some(node) = self.node_stack.last() {
            node.get_node(&name)
        } else {
            self.nodes
                .get(&name)
                .ok_or_else(|| LoadError::NoSuchNode(name.to_string()).into())
        }?;

        self.node_stack.push(new_node);
        Ok(())
    }

    pub fn end_node(&mut self) -> anyhow::Result<()> {
        self.node_stack.pop().ok_or(LoadError::NoNodeToEnd)?;
        Ok(())
    }

    pub fn get_property(&self, name: String) -> anyhow::Result<&Vec<String>> {
        if let Some(node) = self.node_stack.last() {
            node.get_property(name)
        } else {
            Err(LoadError::NoNodeToGetProperty(name).into())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoadNode {
    subnodes: HashMap<String, Box<LoadNode>>,
    properties: HashMap<String, Vec<String>>,
}

impl LoadNode {
    pub fn get_node(&self, name: &str) -> anyhow::Result<&LoadNode> {
        self.subnodes
            .get(name)
            .map(|n| n.as_ref())
            .ok_or_else(|| LoadError::NoSuchNode(name.to_string()).into())
    }

    pub fn get_property(&self, name: String) -> anyhow::Result<&Vec<String>> {
        self.properties
            .get(&name)
            .ok_or_else(|| LoadError::NoSuchProperty(name).into())
    }
}
