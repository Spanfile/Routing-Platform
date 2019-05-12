use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Value {
    #[serde(rename = "literal")]
    Literal(String),
    #[serde(rename = "template")]
    Template(String),
    #[serde(rename = "range")]
    Range { lower: Bound, upper: Bound },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Bound {
    #[serde(rename = "inclusive")]
    Inclusive(f64),
    #[serde(rename = "exclusive")]
    Exclusive(f64),
}
