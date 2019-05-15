use super::{Schema, Validate, ValidationError};
use regex_automata::{DenseDFA, DFA};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub regex: String,
    #[serde(skip)]
    compiled_regex: RefCell<Option<DenseDFA<Vec<usize>, usize>>>,
}

impl Template {
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

impl Template {
    pub fn matches(&self, value: &String) -> bool {
        self.compile_regex();
        let regex_option = self.compiled_regex.borrow();
        let regex = regex_option
            .as_ref()
            .expect("compiled regex cache empty while checking match");
        regex.is_match(value.as_bytes())
    }
}

impl Validate for Template {
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
