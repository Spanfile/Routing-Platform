use crate::schema::{Matches, Schema, Validate, ValidationError};
use regex_automata::{DenseDFA, DFA};
use serde::de::{self, Visitor};
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use std::cell::RefCell;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RegexTemplate {
    regex: String,
    compiled_regex: RefCell<Option<DenseDFA<Vec<usize>, usize>>>,
}

impl RegexTemplate {
    pub fn serialise_regex(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        self.compile_regex();
        let bytes = self
            .compiled_regex
            .borrow()
            .as_ref()
            .expect("compiled regex cache empty while serialising")
            .to_u16()?
            .to_bytes_native_endian()?;
        Ok(bytes)
    }

    pub fn compile_regex(&self) {
        if self.compiled_regex.borrow().is_none() {
            *self.compiled_regex.borrow_mut() =
                Some(DenseDFA::new(&self.regex).expect("regex compilation failed"));
        }
    }

    pub fn deserialise_regex(&self, bytes: &Vec<u8>) {
        let dfa: DenseDFA<Vec<u16>, u16> = unsafe { DenseDFA::from_bytes(bytes).to_owned() };
        *self.compiled_regex.borrow_mut() =
            Some(dfa.to_sized().expect("couldn't create new usize DFA"));
    }

    pub fn compiled_regex_size(&self) -> usize {
        match self.compiled_regex.borrow().as_ref() {
            Some(dfa) => dfa.memory_usage(),
            None => 0,
        }
    }
}

impl Matches for RegexTemplate {
    fn matches(&self, value: &String) -> bool {
        self.compile_regex();
        let regex_option = self.compiled_regex.borrow();
        let regex = regex_option
            .as_ref()
            .expect("compiled regex cache empty while checking match");
        regex.is_match(value.as_bytes())
    }
}

impl Validate for RegexTemplate {
    fn validate(&self, _schema: &Schema) -> Vec<ValidationError> {
        match DenseDFA::new(&self.regex) {
            Ok(r) => {
                *self.compiled_regex.borrow_mut() = Some(r);
                vec![]
            }
            Err(e) => vec![ValidationError::new(format!(
                "Template validation error\nRegex: {}\n{}",
                &self.regex,
                e.description()
            ))],
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

// as the deserialisation was directly from the regex string, serialise directly into it as well
impl Serialize for RegexTemplate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&*self.regex)
    }
}
