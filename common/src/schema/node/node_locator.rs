use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct NodeLocator {
    node: String,
    previous: Option<Rc<NodeLocator>>,
}

impl NodeLocator {
    pub fn new(node: String, previous: Option<Rc<NodeLocator>>) -> NodeLocator {
        NodeLocator { node, previous }
    }
}

impl Default for NodeLocator {
    fn default() -> Self {
        NodeLocator {
            node: String::from(""),
            previous: None,
        }
    }
}
