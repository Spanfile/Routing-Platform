use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Query {
    #[serde(rename = "ls")]
    Ls(String),
}
