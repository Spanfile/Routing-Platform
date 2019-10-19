mod regex_template;

use super::{value::range::Range, Matches, Schema, Validate};
use regex_template::RegexTemplate;
use rp_log::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Template {
    #[serde(rename = "regex")]
    Regex(Box<RegexTemplate>),
    #[serde(rename = "range")]
    Range(Range),
}

impl Template {
    pub fn matches(&self, value: &str) -> bool {
        match self {
            Template::Regex(regex) => regex.matches(value),
            Template::Range(range) => range.matches(value),
        }
    }

    pub fn load_regex_from_cache(&self, regex_cache: Option<&Vec<u8>>) {
        if let Template::Regex(regex) = self {
            match regex_cache {
                Some(cache) => regex.deserialise_regex(cache),
                None => {
                    debug!("Missing cached regex for template, recompiling");
                    regex.compile_regex();
                }
            }
        } else {
            warn!("Tried to load regex into non-regex template");
        }
    }

    pub fn serialise_regex(&self) -> anyhow::Result<Option<Vec<u8>>> {
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
    fn validate(&self, schema: &Schema) -> anyhow::Result<()> {
        match self {
            Template::Regex(regex) => regex.validate(schema),
            Template::Range(range) => range.validate(schema),
        }
    }
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Template::Regex(regex_templ) => write!(f, "{}", regex_templ),
            Template::Range(range) => write!(f, "{}", range),
        }
    }
}
