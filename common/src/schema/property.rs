use super::value::{DefaultValue, Value};
use super::{Matches, Schema, Validate, ValidationError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Property {
    #[serde(default)]
    pub multiple: bool,
    #[serde(default)]
    pub default: Vec<DefaultValue>,
    pub values: Vec<Value>,
    #[serde(default = "Property::default_deletable")]
    pub deletable: bool,
}

impl Property {
    fn default_deletable() -> bool {
        true
    }
}

impl Validate for Property {
    fn validate(&self, schema: &Schema) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if self.values.is_empty() {
            errors.push(ValidationError::new(format!(
                "Property validation error\nNo values defined",
            )));
        }

        for value in &self.values {
            match value {
                Value::Template(template) => {
                    if !schema.templates.contains_key(template) {
                        errors.push(ValidationError::new(format!(
                            "Property validation error\nTemplate '{}' for template value doesn't exist",
                            template
                        )));
                    }
                }
                Value::Range(range) => errors.extend(range.validate(schema)),
                _ => (),
            };
        }

        if !self.default.is_empty() {
            if !self.multiple && self.default.len() > 1 {
                errors.push(ValidationError::new(format!(
                    "Property validation error\nMultiple default values given where multiple values is disallowed",
                )));
            }

            let mut match_found = false;
            for default in &self.default {
                match default {
                    DefaultValue::Literal(def) => {
                        for value in &self.values {
                            match value {
                                Value::Literal(literal) => {
                                    if def == literal {
                                        match_found = true;
                                        break;
                                    }
                                }
                                Value::Template(template) => {
                                    // by now it's guaranteed the specified template exists (it has been validated above)
                                    let templ = schema.templates.get(template).expect(&format!(
                                        "template {} not found after existence validation",
                                        template
                                    ));
                                    if templ.matches(def) {
                                        match_found = true;
                                        break;
                                    }
                                }
                                Value::Range(range) => {
                                    if range.matches(def) {
                                        match_found = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    _ => match_found = true,
                }

                if !match_found {
                    errors.push(ValidationError::new(format!(
                        "Property validation error\nNo value allows given default value '{:?}'",
                        default
                    )));
                }
            }
        }

        errors
    }
}
