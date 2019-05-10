use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    pub value: u32,
}

impl Schema {
    pub fn get_binary(self) -> String {
        return serde_json::to_string(&self).unwrap();
    }

    pub fn from_binary(data: String) -> Schema {
        return serde_json::from_str(&data).unwrap();
    }
}
