use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub enum Query {
    #[serde(rename = "ls")]
    Ls(String),
    #[serde(rename = "cat")]
    Cat(String),
}

impl Query {
    pub fn run(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        match self {
            Query::Ls(path) => {
                let mut dirs = Vec::new();
                for entry in fs::read_dir(path)? {
                    let path = entry?.path();
                    let name = path.file_name();
                    dirs.push(String::from(name.unwrap().to_str().unwrap()));
                }
                Ok(dirs)
            }
            Query::Cat(path) => {}
        }
    }
}
