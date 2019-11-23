use crate::{error, Bound, Matches, Schema, Validate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Range {
    pub lower: Bound,
    pub upper: Bound,
}

impl Matches for Range {
    fn matches(&self, value: &str) -> anyhow::Result<bool> {
        value
            .parse::<f64>()
            .map(|v| self.lower.match_against_with(self.upper, v))
            .or(Ok(false))
    }
}

impl Validate for Range {
    fn validate(&self, _schema: &Schema) -> anyhow::Result<()> {
        match (self.lower, self.upper) {
            (Bound::Inclusive(lower_v), Bound::Inclusive(upper_v))
            | (Bound::Exclusive(lower_v), Bound::Inclusive(upper_v)) => {
                if lower_v > upper_v {
                    Err(error::SchemaValidationError::Range {
                        lower: self.lower,
                        upper: self.upper,
                    }
                    .into())
                } else {
                    Ok(())
                }
            }
            (Bound::Inclusive(lower_v), Bound::Exclusive(upper_v))
            | (Bound::Exclusive(lower_v), Bound::Exclusive(upper_v)) => {
                if lower_v >= upper_v {
                    Err(error::SchemaValidationError::Range {
                        lower: self.lower,
                        upper: self.upper,
                    }
                    .into())
                } else {
                    Ok(())
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    #[test]
    fn matches() -> anyhow::Result<()> {
        if !(Range {
            lower: Bound::Inclusive(0.0.into()),
            upper: Bound::Inclusive(1.0.into()),
        })
        .matches("0.5")?
        {
            Err(anyhow!("valid value doesn't match"))
        } else if (Range {
            lower: Bound::Inclusive(0.0.into()),
            upper: Bound::Inclusive(1.0.into()),
        })
        .matches("-1.0")?
        {
            Err(anyhow!("invalid lower value matches"))
        } else if (Range {
            lower: Bound::Inclusive(0.0.into()),
            upper: Bound::Inclusive(1.0.into()),
        })
        .matches("2.0")?
        {
            Err(anyhow!("invalid higher value matches"))
        } else {
            Ok(())
        }
    }
}
