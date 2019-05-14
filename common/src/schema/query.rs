use crate::context::Context;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    pub id: String,
    pub command: Command,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    #[serde(rename = "ls")]
    Ls(String),
    #[serde(rename = "cat")]
    Cat(String),
}

impl Query {
    pub fn run(&self, context: &Context) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        match &self.command {
            Command::Ls(path) => {
                let path = context
                    .format(path.to_owned())
                    .expect("couldn't context format query ls command path");
                let mut dirs = Vec::new();

                for entry in fs::read_dir(path)? {
                    let path = entry?.path();
                    let name = path.file_name();
                    dirs.push(String::from(name.unwrap().to_str().unwrap()));
                }
                Ok(dirs)
            }
            Command::Cat(path) => {
                let path = context
                    .format(path.to_owned())
                    .expect("couldn't context format query cat command path");

                Ok(vec![fs::read_to_string(&path)
                    .expect(&format!(
                        "couldn't read file {} for query cat command",
                        path
                    ))
                    .trim()
                    .to_owned()])
            }
        }
    }
}
