use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    pub templates: Vec<Template>,
    pub nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub name: String,
    pub regex: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Node {
    pub name: String,
    #[serde(default)]
    pub subnodes: Vec<Box<Node>>,
    #[serde(default)]
    pub properties: Vec<Property>,
    #[serde(default)]
    pub query: Option<Query>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Property {
    pub key: String,
    #[serde(default)]
    pub multiple: bool,
    pub values: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Query {
    #[serde(rename = "ls")]
    Ls(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Value {
    #[serde(rename = "literal")]
    Literal(String),
    #[serde(rename = "template")]
    Template(String),
    #[serde(rename = "range")]
    Range { from: Bound, to: Bound },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Bound {
    #[serde(rename = "inclusive")]
    Inclusive(i64),
    #[serde(rename = "exclusive")]
    Exclusive(i64),
}

impl Schema {
    pub fn to_binary_file(self, file: &mut File) -> Result<(), serde_cbor::error::Error> {
        serde_cbor::to_writer(file, &self)
    }

    pub fn from_yaml_file(file: &File) -> Result<Schema, Box<Error>> {
        let reader = BufReader::new(file);
        Ok(serde_yaml::from_reader(reader)?)
    }

    pub fn from_binary_file(file: &File) -> Result<Schema, Box<Error>> {
        let reader = BufReader::new(file);
        Ok(serde_cbor::from_reader(reader)?)
    }
}
