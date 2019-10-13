use crate::{error, Bound, Matches, Schema, Validate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Range {
    pub lower: Bound,
    pub upper: Bound,
}

impl Matches for Range {
    fn matches(&self, value: &str) -> bool {
        match value.parse::<f64>() {
            Ok(v) => {
                (match self.lower {
                    Bound::Inclusive(bound) => v >= bound,
                    Bound::Exclusive(bound) => v > bound,
                }) && (match self.upper {
                    Bound::Inclusive(bound) => v <= bound,
                    Bound::Exclusive(bound) => v < bound,
                })
            }
            Err(_e) => false,
        }
    }
}

impl Validate for Range {
    fn validate(&self, _schema: &Schema) -> error::Result<()> {
        match self.lower {
            Bound::Inclusive(lower_v) => match self.upper {
                Bound::Inclusive(upper_v) => {
                    if upper_v < lower_v {
                        Err(error::SchemaValidationError::Range {
                            lower: self.lower,
                            upper: self.upper,
                            source: None,
                        }
                        .into())
                    } else {
                        Ok(())
                    }
                }
                Bound::Exclusive(upper_v) => {
                    if upper_v < lower_v {
                        Err(error::SchemaValidationError::Range {
                            lower: self.lower,
                            upper: self.upper,
                            source: None,
                        }
                        .into())
                    } else {
                        Ok(())
                    }
                }
            },
            Bound::Exclusive(lower_v) => match self.upper {
                Bound::Inclusive(upper_v) => {
                    if upper_v <= lower_v {
                        Err(error::SchemaValidationError::Range {
                            lower: self.lower,
                            upper: self.upper,
                            source: None,
                        }
                        .into())
                    } else {
                        Ok(())
                    }
                }
                Bound::Exclusive(upper_v) => {
                    if upper_v <= lower_v {
                        Err(error::SchemaValidationError::Range {
                            lower: self.lower,
                            upper: self.upper,
                            source: None,
                        }
                        .into())
                    } else {
                        Ok(())
                    }
                }
            },
        }
    }
}

impl std::fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.lower {
            Bound::Inclusive(v) => write!(f, "[{}, ", v)?,
            Bound::Exclusive(v) => write!(f, "]{}, ", v)?,
        };
        match self.upper {
            Bound::Inclusive(v) => write!(f, "{}]", v),
            Bound::Exclusive(v) => write!(f, "{}[", v),
        }
    }
}
