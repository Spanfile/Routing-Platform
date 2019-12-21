use super::{Matches, Schema, Validate};
use crate::error;
use anyhow::anyhow;
use regex_automata::{DenseDFA, DFA};
use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::{cell::RefCell, error::Error, fmt};

#[derive(Debug)]
pub struct RegexTemplate {
    pub regex: String,
    compiled_regex: RefCell<Option<DenseDFA<Vec<usize>, usize>>>,
}

impl RegexTemplate {
    pub fn serialise_regex(&self) -> anyhow::Result<Vec<u8>> {
        self.compile_regex()?;
        let bytes = self
            .compiled_regex
            .borrow()
            .as_ref()
            .ok_or_else(|| anyhow!("Compiled regex cache empty while serialising"))?
            .to_u16()?
            .to_bytes_native_endian()?;
        Ok(bytes)
    }

    pub fn compile_regex(&self) -> anyhow::Result<()> {
        if self.compiled_regex.borrow().is_none() {
            *self.compiled_regex.borrow_mut() = Some(DenseDFA::new(&self.regex)?);
        }
        Ok(())
    }

    pub fn deserialise_regex(&self, bytes: &[u8]) -> anyhow::Result<()> {
        let dfa: DenseDFA<Vec<u16>, u16> = unsafe { DenseDFA::from_bytes(bytes).to_owned() };
        *self.compiled_regex.borrow_mut() = Some(dfa.to_sized()?);
        Ok(())
    }

    pub fn compiled_regex_size(&self) -> usize {
        match self.compiled_regex.borrow().as_ref() {
            Some(dfa) => dfa.memory_usage(),
            None => 0,
        }
    }
}

impl Matches for RegexTemplate {
    fn matches(&self, value: &str) -> anyhow::Result<bool> {
        self.compile_regex()?;
        let regex_option = self.compiled_regex.borrow();
        let regex = regex_option
            .as_ref()
            .ok_or_else(|| anyhow!("Compiled regex cache empty while checking match"))?;
        Ok(regex.is_match(value.as_bytes()))
    }
}

impl Validate for RegexTemplate {
    fn validate(&self, _schema: &Schema) -> anyhow::Result<()> {
        match DenseDFA::new(&self.regex) {
            Ok(r) => {
                *self.compiled_regex.borrow_mut() = Some(r);
                Ok(())
            }
            Err(e) => Err(error::SchemaValidationError::Regex {
                regex: self.regex.to_owned(),
                description: e.description().to_string(),
            }
            .into()),
        }
    }
}

// this allows a regex template be deserialised directly from a regex string
struct RegexTemplateVisitor;
impl<'de> Visitor<'de> for RegexTemplateVisitor {
    type Value = RegexTemplate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RegexTemplate {
            regex: value.to_owned(),
            compiled_regex: RefCell::new(None),
        })
    }
}

impl<'de> Deserialize<'de> for RegexTemplate {
    fn deserialize<D>(deserializer: D) -> Result<RegexTemplate, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RegexTemplateVisitor)
    }
}

// as the deserialisation was directly from the regex string, serialise directly
// into it as well
impl Serialize for RegexTemplate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&*self.regex)
    }
}

impl std::fmt::Display for RegexTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.regex)
    }
}
