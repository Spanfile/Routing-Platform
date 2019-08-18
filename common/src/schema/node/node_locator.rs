#[derive(Debug)]
pub struct NodeLocator {
    node: String,
    next: Option<Box<NodeLocator>>,
}

impl NodeLocator {
    pub fn new(node: String, next: Option<NodeLocator>) -> NodeLocator {
        NodeLocator {
            node,
            next: next.map(Box::new),
        }
    }
}
