use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bound {
    #[serde(rename = "inclusive")]
    Inclusive(OrderedFloat<f64>),
    #[serde(rename = "exclusive")]
    Exclusive(OrderedFloat<f64>),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_against() {
        let lower_inclusive = Bound::Inclusive(1.0);
        let lower_exclusive = Bound::Exclusive(1.0);
        let upper_inclusive = Bound::Inclusive(2.0);
        let upper_exclusive = Bound::Exclusive(2.0);

        assert!(lower_inclusive.match_against_with(upper_inclusive, 1.0));
        assert!(lower_inclusive.match_against_with(upper_exclusive, 1.0));

        assert!(lower_exclusive.match_against_with(upper_inclusive, 1.5));
        assert!(lower_exclusive.match_against_with(upper_exclusive, 1.5));
    }
}
