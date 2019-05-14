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
        let mut values = Vec::new();

        for default in &property.default {
            values.extend(match default.resolve() {
                Ok(v) => v,
                Err(e) => {
                    println!(
                        "Error while resolving default value for property: {:?} (parent path: {})\n{:?}",
                        property, parent, e
                    );
                    vec![]
                }
            });
        }

        Property {
            key: property.key.clone(),
            path: parent.clone(),
            values: values,
        }
    }
}
