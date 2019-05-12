#[derive(Debug)]
pub struct Property {
    pub key: String,
    pub path: String,
    pub values: Vec<String>,
}

impl Property {
    pub fn from_schema_property(
        parent: &String,
        property: &crate::schema::property::Property,
    ) -> Property {
        Property {
            key: property.key.clone(),
            path: parent.clone(),
            values: property.default.clone(),
        }
    }
}
