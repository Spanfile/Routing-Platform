use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Bound {
    #[serde(rename = "inclusive")]
    Inclusive(f64),
    #[serde(rename = "exclusive")]
    Exclusive(f64),
}

impl Bound {
    pub fn match_against_with(&self, other: Bound, value: f64) -> bool {
        (match self {
            Bound::Inclusive(bound) => value >= *bound,
            Bound::Exclusive(bound) => value > *bound,
        }) && (match other {
            Bound::Inclusive(bound) => value <= bound,
            Bound::Exclusive(bound) => value < bound,
        })
    }
}
