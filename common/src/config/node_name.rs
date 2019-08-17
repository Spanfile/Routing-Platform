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
