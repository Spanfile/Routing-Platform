use super::{Schema, Validate, ValidationError};
use regex_automata::{DenseDFA, Regex};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub name: String,
    pub regex: String,
    #[serde(skip)]
    compiled_regex: RefCell<Option<Regex>>,
}

impl Template {
    pub fn serialise_regex(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let bytes = DenseDFA::new(&self.regex)?
            .to_u16()?
            .to_bytes_native_endian()?;
        Ok(bytes)
    }

    fn compile_regex(&self) {
        if self.compiled_regex.borrow().is_none() {
            *self.compiled_regex.borrow_mut() =
                Some(Regex::new(&self.regex).expect("regex compilation failed"));
        }
    }

    pub fn matches(&self, value: &String) -> bool {
        self.compile_regex();
        let regex_option = self.compiled_regex.borrow();
        let regex = regex_option.as_ref().expect("compiled regex cache empty");
        regex.is_match(value.as_bytes())
    }
}

impl Validate for Template {
    fn validate(&self, _schema: &Schema) -> Vec<ValidationError> {
        match Regex::new(&self.regex) {
            Ok(r) => {
                *self.compiled_regex.borrow_mut() = Some(r);
                vec![]
            }
            Err(e) => vec![ValidationError::new(format!(
                "Template validation error\nName: {}\nRegex: {}\n{}",
                &self.name,
                &self.regex,
                e.description()
            ))],
        }
    }
}
