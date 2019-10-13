use rp_error::ErrorTrait;

#[derive(Debug)]
pub enum ConfigEditorError {
    NodeNotFound {
        node: String,
        source: Option<Box<dyn ErrorTrait>>,
    },
    PropertyNotFound {
        property: String,
        source: Option<Box<dyn ErrorTrait>>,
    },
    AlreadyAtTop {
        source: Option<Box<dyn ErrorTrait>>,
    },
    ValueError {
        source: Option<Box<dyn ErrorTrait>>,
    },
    AmbiguousNodeName {
        name: String,
        source: Option<Box<dyn ErrorTrait>>,
    },
}

impl ErrorTrait for ConfigEditorError {
    fn display(&self) -> String {
        match self {
            ConfigEditorError::NodeNotFound { node, .. } => format!("No such node: '{}'", node),
            ConfigEditorError::PropertyNotFound { property, .. } => {
                format!("No such property: '{}'", property)
            }
            ConfigEditorError::AlreadyAtTop { .. } => String::from("Already at top"),
            ConfigEditorError::ValueError { .. } => String::from("Invalid value"),
            ConfigEditorError::AmbiguousNodeName { name, .. } => format!(
                "Ambiguous node name: '{}' (multiple literal node names)",
                name
            ),
        }
    }

    fn source(&self) -> Option<&dyn ErrorTrait> {
        match &self {
            ConfigEditorError::NodeNotFound { source, .. } => source.as_deref(),
            ConfigEditorError::PropertyNotFound { source, .. } => source.as_deref(),
            ConfigEditorError::AlreadyAtTop { source, .. } => source.as_deref(),
            ConfigEditorError::ValueError { source } => source.as_deref(),
            ConfigEditorError::AmbiguousNodeName { source, .. } => source.as_deref(),
        }
    }
}
