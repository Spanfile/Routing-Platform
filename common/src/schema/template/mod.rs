mod regex_template;

use super::{value::range::Range, Schema, Validate, ValidationError};
use regex_template::RegexTemplate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Template {
    #[serde(rename = "regex")]
    Regex(RegexTemplate),
    #[serde(rename = "range")]
    Range(Range),
}

trait Matches {
    fn matches(&self, value: &String) -> bool;
}

impl Template {
    pub fn matches(&self, value: &String) -> bool {
        match self {
            Template::Regex(regex) => regex.matches(value),
            Template::Range(range) => match value.parse() {
                Ok(v) => range.matches(v),
                Err(_e) => false,
            },
        }
    }

    pub fn load_regex_from_cache(&self, regex_cache: Option<&Vec<u8>>) {
        match self {
            Template::Regex(regex) => match regex_cache {
                Some(cache) => regex.deserialise_regex(cache),
                None => {
                    println!("missing cached regex for template, recompiling");
                    regex.compile_regex();
                }
            },
            _ => (),
        }
    }

    pub fn serialise_regex(&self) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        match self {
            Template::Regex(regex) => Ok(Some(regex.serialise_regex()?)),
            _ => Ok(None),
        }
    }

    pub fn compiled_regex_size(&self) -> usize {
        match self {
            Template::Regex(regex) => regex.compiled_regex_size(),
            _ => 0,
        }
    }
}

impl Validate for Template {
    fn validate(&self, schema: &Schema) -> Vec<ValidationError> {
        match self {
            Template::Regex(regex) => regex.validate(schema),
            Template::Range(range) => range.validate(schema),
        }
    }
}
