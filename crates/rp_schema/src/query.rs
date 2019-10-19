use rp_common::Context;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Query {
    pub id: String,
    pub command: Command,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Command {
    #[serde(rename = "ls")]
    Ls(String),
    #[serde(rename = "cat")]
    Cat(String),
}

impl Query {
    pub fn run(&self, context: &Context) -> anyhow::Result<Vec<String>> {
        match &self.command {
            Command::Ls(path) => {
                let path = context.format(format!("{}{}", "{mock}", path.to_owned()))?;
                // println!("{}", path);
                let mut dirs = Vec::new();

                for entry in fs::read_dir(path)? {
                    let path = entry?.path();
                    let name = path.file_name();
                    dirs.push(String::from(name.unwrap().to_str().unwrap()));
                }
                Ok(dirs)
            }
            Command::Cat(path) => {
                let path = context.format(format!("{}{}", "{mock}", path.to_owned()))?;
                // println!("{}", path);
                Ok(vec![fs::read_to_string(&path)?.trim().to_owned()])
            }
        }
    }
}
