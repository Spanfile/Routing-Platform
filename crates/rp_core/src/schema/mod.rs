mod bound;
mod matches;
mod merge;
mod node;
mod property;
mod query;
mod template;
#[cfg(test)]
mod tests;
mod validate;
mod value;

use crate::{error, log::*};
pub use bound::Bound;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
pub use matches::Matches;
pub use merge::{Merge, MergingStrategy};
pub use node::{
    MultiSchemaNode, MultiSchemaNodeSource, NodeLocator, SchemaNode, SchemaNodeTrait,
    SingleSchemaNode,
};
pub use property::Property;
pub use query::Query;
use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map::Entry, HashMap},
    io::{BufReader, Read, Write},
    rc::Rc,
};
pub use template::Template;
pub use validate::Validate;
pub use value::{DefaultValue, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    pub templates: HashMap<String, Rc<Template>>,
    pub nodes: HashMap<String, Box<SchemaNode>>,
    #[serde(default)]
    regex_cache: HashMap<String, Vec<u8>>,
}

impl Schema {
    pub fn to_binary_file<W: Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        let encoder = ZlibEncoder::new(writer, Compression::best());
        Ok(serde_json::to_writer(encoder, &self)?)
    }

    pub fn from_yaml_file<R: Read>(reader: R) -> anyhow::Result<Schema> {
        let buf_reader = BufReader::new(reader);
        Ok(serde_yaml::from_reader(buf_reader)?)
    }

    pub fn from_binary(binary: &[u8]) -> anyhow::Result<Schema> {
        let decoder = ZlibDecoder::new(binary);
        let mut schema: Schema = serde_json::from_reader(decoder)?;
        schema.load_regexes_from_cache()?;
        schema.populate_node_metadata();
        Ok(schema)
    }
}

impl Schema {
    pub fn validate(&mut self) -> anyhow::Result<()> {
        self.validate_templates()?;
        self.validate_nodes()?;
        Ok(())
    }

    fn validate_templates(&mut self) -> anyhow::Result<()> {
        for template in self.templates.values() {
            template.validate(&self)?;
        }
        Ok(())
    }

    fn validate_nodes(&self) -> anyhow::Result<()> {
        for node in self.nodes.values() {
            node.validate(&self)?;
        }

        Ok(())
    }
}

impl Schema {
    pub fn build_regex_cache(&mut self) -> anyhow::Result<()> {
        self.regex_cache = HashMap::new();

        for (name, template) in &self.templates {
            if let Some(bytes) = template.serialise_regex()? {
                self.regex_cache.insert(name.clone(), bytes);
            }
        }

        Ok(())
    }

    fn populate_node_metadata(&mut self) {
        let root_locator = Rc::new(NodeLocator::new(String::from("schema"), None));
        for (name, node) in self.nodes.iter_mut() {
            node.update_locators(name.to_owned(), Rc::clone(&root_locator));
        }
    }

    fn load_regexes_from_cache(&self) -> anyhow::Result<()> {
        for (name, template) in &self.templates {
            template.load_regex_from_cache(self.regex_cache.get(name))?;
        }

        Ok(())
    }
}

impl Schema {
    pub fn print_trace_info(&self) {
        trace!("Schema templates: {}", self.templates.len());
        trace!("Schema nodes: {}", self.node_count());
        trace!("Schema properties: {}", self.property_count());
        trace!(
            "Schema regex cache DFA size: {} bytes",
            self.regex_cache_dfa_size()
        );
    }

    fn regex_cache_dfa_size(&self) -> usize {
        let mut sum = 0;
        for template in self.templates.values() {
            sum += template.compiled_regex_size();
        }
        sum
    }

    fn node_count(&self) -> usize {
        let mut sum = 0;
        for node in self.nodes.values() {
            sum += node.node_count();
        }
        sum
    }

    fn property_count(&self) -> usize {
        let mut sum = 0;
        for node in self.nodes.values() {
            sum += node.property_count();
        }
        sum
    }

    pub fn find_node(&self, locator: Rc<NodeLocator>) -> Option<&SchemaNode> {
        let mut locator_stack = Vec::new();
        let mut current = Some(locator);

        while let Some(c) = &current {
            locator_stack.push(c.node.to_owned());
            current = if let Some(prev_rc) = &c.previous {
                Some(Rc::clone(prev_rc))
            } else {
                None
            };
        }

        // the first locator is always "schema", i.e. this
        locator_stack.pop();

        // find the first node in schema
        let mut current = if let Some(name) = &locator_stack.pop() {
            self.nodes.get(name).map(|n| &**n)
        } else {
            None
        };

        current?;

        let mut name = locator_stack.pop();

        while let Some(n) = &name {
            current = if let Some(c) = current {
                match c {
                    SchemaNode::SingleSchemaNode(single) => single.subnodes.get(n).map(|n| &**n),
                    SchemaNode::MultiSchemaNode(multi) => Some(&multi.node),
                }
            } else {
                None
            };

            current?;
            name = locator_stack.pop();
        }

        current
    }
}

impl Merge for Schema {
    fn merge(&mut self, other: Self, strategy: MergingStrategy) -> anyhow::Result<()> {
        for (name, template) in other.templates {
            match self.templates.entry(name) {
                Entry::Occupied(mut existing) => match strategy {
                    MergingStrategy::Ours => (),
                    MergingStrategy::Theirs => {
                        existing.insert(template);
                    }
                    MergingStrategy::Error => {
                        return Err(error::MergeError::Conflict {
                            this: format!("{:?}", existing),
                            that: format!("{:?}", template),
                        }
                        .into())
                    }
                },
                Entry::Vacant(existing) => {
                    existing.insert(template);
                }
            }
        }

        for (name, node) in other.nodes {
            match self.nodes.entry(name) {
                Entry::Occupied(mut existing) => {
                    existing.get_mut().merge(*node, strategy)?;
                }
                Entry::Vacant(existing) => {
                    existing.insert(node);
                }
            }
        }

        Ok(())
    }
}
