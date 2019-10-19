use anyhow::anyhow;
use rp_schema::{Matches, Template};
use std::rc::Weak;

#[derive(Debug)]
pub enum NodeName {
    Literal(String),
    Multiple(Weak<Template>),
}

impl NodeName {
    pub fn matches(&self, name: &str) -> anyhow::Result<bool> {
        match &self {
            NodeName::Literal(s) => Ok(name == s),
            NodeName::Multiple(t) => {
                if let Some(template) = t.upgrade() {
                    template.matches(name)
                } else {
                    Err(anyhow!(
                        "Weak template reference in NodeName failed upgrade"
                    ))
                }
            }
        }
    }
}

impl std::fmt::Display for NodeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeName::Literal(s) => write!(f, "{}", s),
            NodeName::Multiple(templ) => {
                write!(f, "{}", templ.upgrade().expect("template dropped"))
            }
        }
    }
}
