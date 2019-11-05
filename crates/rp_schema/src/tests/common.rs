use super::Schema;
use std::io::{Seek, SeekFrom, Write};
use tempfile::tempfile;

pub fn get_valid_schema() -> anyhow::Result<Schema> {
    let mut temp = tempfile()?;
    let schema = r#"---
templates:
  "string":
    regex: ".*"
nodes:
  "system":
    properties:
      "hostname":
        default:
          - literal: router
        deletable: false
        values:
          - template: string
    subnodes:
      "ntp":
        properties:
          "server":
            multiple: true
            default:
              - literal: 1.pool.ntp.org
            values:
              - template: string"#;

    write!(temp, "{}", schema)?;
    temp.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(&temp)
}

pub fn get_invalid_template_schema() -> anyhow::Result<Schema> {
    let mut temp = tempfile()?;
    let schema = r#"---
templates:
  "string":
    regex: "*"
nodes: {}"#;

    write!(temp, "{}", schema)?;
    temp.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(&temp)
}

pub fn get_invalid_node_schema() -> anyhow::Result<Schema> {
    let mut temp = tempfile()?;
    let schema = r#"---
templates: {}
nodes:
    "invalid":
        properties:
            "invalid":
                values: []"#;

    write!(temp, "{}", schema)?;
    temp.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(&temp)
}
