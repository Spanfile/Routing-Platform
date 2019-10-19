use rp_log::*;
use rp_schema::Template;
use std::rc::Weak;

#[derive(Debug)]
pub enum NodeName {
    Literal(String),
    Multiple(Weak<Template>),
}

impl NodeName {
    pub fn matches(&self, name: &str) -> bool {
        match &self {
            NodeName::Literal(s) => name == s,
            NodeName::Multiple(_t) => {
                // TODO: actually check the template
                warn!("NodeName matching against a template not implemented yet");
                true
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
