use crate::schema::{Schema, Validate, ValidationError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Range {
    pub lower: Bound,
    pub upper: Bound,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Bound {
    #[serde(rename = "inclusive")]
    Inclusive(f64),
    #[serde(rename = "exclusive")]
    Exclusive(f64),
}

impl Range {
    pub fn matches(&self, value: f64) -> bool {
        (match self.lower {
            Bound::Inclusive(bound) => value >= bound,
            Bound::Exclusive(bound) => value > bound,
        }) && (match self.upper {
            Bound::Inclusive(bound) => value <= bound,
            Bound::Exclusive(bound) => value < bound,
        })
    }
}

impl Validate for Range {
    fn validate(&self, _schema: &Schema) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        match self.lower {
            Bound::Inclusive(lower_v) => match self.upper {
                Bound::Inclusive(upper_v) => {
                    if upper_v < lower_v {
                        errors.push(ValidationError::new(format!(
                            "Range validation error\nInclusive upper bound {} lower than inclusive lower bound {}",
                            upper_v,
                            lower_v
                        )));
                    }
                }
                Bound::Exclusive(upper_v) => {
                    if upper_v < lower_v {
                        errors.push(ValidationError::new(format!(
                            "Range validation error\nExclusive upper bound {} lower than inclusive lower bound {}",
                            upper_v,
                            lower_v
                        )));
                    }
                }
            },
            Bound::Exclusive(lower_v) => match self.upper {
                Bound::Inclusive(upper_v) => {
                    if upper_v <= lower_v {
                        errors.push(ValidationError::new(format!(
                            "Range validation error\nInclusive upper bound {} lower or equal to exclusive lower bound {}",
                            upper_v,
                            lower_v
                        )));
                    }
                }
                Bound::Exclusive(upper_v) => {
                    if upper_v <= lower_v {
                        errors.push(ValidationError::new(format!(
                            "Range validation error\nExclusive upper bound {} lower or equal to exclusive lower bound {}",
                            upper_v,
                            lower_v
                        )));
                    }
                }
            },
        }
        errors
    }
}
