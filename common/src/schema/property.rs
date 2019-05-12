use super::value::{Bound, Value};
use super::{Schema, Validate, ValidationError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Property {
    pub key: String,
    #[serde(default)]
    pub multiple: bool,
    #[serde(default)]
    pub default: Vec<String>,
    pub values: Vec<Value>,
}

impl Validate for Property {
    fn validate(&self, schema: &Schema) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if self.key.is_empty() {
            errors.push(ValidationError::new(format!(
                "Property validation error\nEmpty key",
            )));
        }

        if self.values.is_empty() {
            errors.push(ValidationError::new(format!(
                "Property validation error\nKey: {}\nNo values defined",
                self.key
            )));
        }

        for value in &self.values {
            match value {
                Value::Template(template) => {
                    let templ = schema.templates.iter().find(|&t| &t.name == template);
                    if templ.is_none() {
                        errors.push(ValidationError::new(format!(
                            "Property validation error\nKey: {}\nTemplate '{}' for template value doesn't exist",
                            self.key,
                            template
                        )));
                    }
                }
                Value::Range { lower, upper } => match lower {
                    Bound::Inclusive(lower_v) => match upper {
                        Bound::Inclusive(upper_v) => {
                            if upper_v < lower_v {
                                errors.push(ValidationError::new(format!(
                                    "Property validation error\nKey: {}\nValue: {:?}\nInclusive upper bound {} lower than inclusive lower bound {}",
                                    self.key,
                                    value,
                                    upper_v,
                                    lower_v
                                )));
                            }
                        }
                        Bound::Exclusive(upper_v) => {
                            if upper_v < lower_v {
                                errors.push(ValidationError::new(format!(
                                    "Property validation error\nKey: {}\nValue: {:?}\nExclusive upper bound {} lower than inclusive lower bound {}",
                                    self.key,
                                    value,
                                    upper_v,
                                    lower_v
                                )));
                            }
                        }
                    },
                    Bound::Exclusive(lower_v) => match upper {
                        Bound::Inclusive(upper_v) => {
                            if upper_v <= lower_v {
                                errors.push(ValidationError::new(format!(
                                    "Property validation error\nKey: {}\nValue: {:?}\nInclusive upper bound {} lower or equal to exclusive lower bound {}",
                                    self.key,
                                    value,
                                    upper_v,
                                    lower_v
                                )));
                            }
                        }
                        Bound::Exclusive(upper_v) => {
                            if upper_v <= lower_v {
                                errors.push(ValidationError::new(format!(
                                    "Property validation error\nKey: {}\nValue: {:?}\nExclusive upper bound {} lower or equal to exclusive lower bound {}",
                                    self.key,
                                    value,
                                    upper_v,
                                    lower_v
                                )));
                            }
                        }
                    },
                },
                _ => (),
            };
        }

        if !self.default.is_empty() {
            if !self.multiple && self.default.len() > 1 {
                errors.push(ValidationError::new(format!(
                    "Property validation error\nKey: {}\nMultiple default values given where multiple values is disallowed",
                    self.key
                )));
            }

            let mut match_found = false;
            for default in &self.default {
                for value in &self.values {
                    match value {
                        Value::Literal(literal) => {
                            if default == literal {
                                match_found = true;
                                break;
                            }
                        }
                        Value::Template(template) => {
                            // by now it's guaranteed the specified template exists (it has been validated above)
                            let templ = schema
                                .templates
                                .iter()
                                .find(|&t| &t.name == template)
                                .unwrap();
                            if templ.matches(default) {
                                match_found = true;
                                break;
                            }
                        }
                        Value::Range { lower, upper } => {
                            let numeric_default: f64 = match default.parse() {
                                Ok(v) => v,
                                Err(_e) => break, // not being an integer means the range isn't valid but another value could still be
                            };

                            let matches_from = match lower {
                                Bound::Inclusive(v) => numeric_default >= *v,
                                Bound::Exclusive(v) => numeric_default > *v,
                            };
                            let matches_to = match upper {
                                Bound::Inclusive(v) => numeric_default <= *v,
                                Bound::Exclusive(v) => numeric_default < *v,
                            };

                            if matches_from && matches_to {
                                match_found = true;
                                break;
                            }
                        }
                    }
                }

                if !match_found {
                    errors.push(ValidationError::new(format!(
                        "Property validation error\nKey: {}\nNo value allows given default value '{}'",
                        self.key,
                        default
                    )));
                }
            }
        }

        errors
    }
}
