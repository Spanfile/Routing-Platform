use crate::schema::Template;

pub enum NodeName<'a> {
    Literal(String),
    Multiple(&'a Template),
}

impl<'a> NodeName<'a> {
    pub fn matches(&self, name: &str) -> bool {
        match &self {
            NodeName::Literal(s) => name == s,
            NodeName::Multiple(_t) => {
                // TODO: actually check the template
                true
            }
        }
    }
}

impl std::fmt::Display for NodeName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeName::Literal(s) => write!(f, "{}", s),
            NodeName::Multiple(templ) => write!(f, "{}", templ),
        }
    }
}
