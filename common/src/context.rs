use lazy_static::lazy_static;
use regex_automata::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Context<'a> {
    values: HashMap<String, String>,
    parent: Option<&'a Context<'a>>,
}

#[derive(Debug)]
pub enum FormatError {
    FormatStringEmpty,
    IDNotInContext(String),
}

impl std::fmt::Display for FormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            FormatError::FormatStringEmpty => write!(f, "format string empty"),
            FormatError::IDNotInContext(_id) => write!(f, "ID not in given context"),
        }
    }
}

impl std::error::Error for FormatError {}

impl<'a> Clone for Context<'a> {
    fn clone(&self) -> Self {
        Context {
            values: self.values.clone(),
            parent: if let Some(parent) = &self.parent {
                Some(parent.clone())
            } else {
                None
            }
        }
    }
}

impl<'a> Context<'a> {
    pub fn new(parent: Option<&'a Context>) -> Context<'a> {
        Context {
            values: HashMap::new(),
            parent,
        }
    }

    pub fn inherit(&mut self, parent: &'a Context) {
        self.parent = Some(parent);
    }

    pub fn get_value(&self, id: &str) -> Option<String> {
        match &self.values.get(id) {
            Some(value) => Some(value.to_string()),
            None => match self.parent {
                Some(p) => p.get_value(id),
                None => None,
            },
        }
    }

    pub fn set_value(&mut self, id: String, value: String) {
        &self.values.insert(id, value);
    }

    pub fn format(&self, text: String) -> Result<String, FormatError> {
        lazy_static! {
            static ref FORMAT_MATCHER: Regex =
                Regex::new(r"\{[^\{\}]*\}").expect("couldn't compile format matcher regex");
        }

        let mut replacements: Vec<(usize, usize, String)> = Vec::new();

        for mat in FORMAT_MATCHER.find_iter(&text.as_bytes()) {
            // right bound being one character away from the left bound means the format string is empty
            if mat.0 == mat.1 - 1 {
                return Err(FormatError::FormatStringEmpty);
            } else {
                let match_str = &text[mat.0 + 1..mat.1 - 1];
                match &self.get_value(match_str) {
                    Some(value) => replacements.push((mat.0, mat.1, value.to_owned())),
                    None => return Err(FormatError::IDNotInContext(match_str.to_owned())),
                };
            }
        }

        // println!("{:?}", replacements);

        let mut text = text;
        let mut len_diff: i32 = 0;

        for replacement in &replacements {
            let left_bound = replacement.0 as i32 + len_diff;
            let right_bound = replacement.1 as i32 + len_diff;
            let value = &replacement.2;
            // println!("{}", text);
            // print!("{:>1$}", "^", left_bound as usize);
            // println!("{:>1$}", "^", (right_bound - left_bound) as usize);
            // println!("len diff: {}", len_diff);

            let orig_len = right_bound - left_bound;
            let new_len = value.len() as i32;
            len_diff += new_len - orig_len;

            let left = &text[..left_bound as usize];
            let right = &text[right_bound as usize..];
            text = format!("{}{}{}", left, value, right);
        }

        Ok(text)
    }
}
