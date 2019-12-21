use super::Schema;
use std::io::{Cursor, Seek, SeekFrom, Write};

fn buffer() -> Cursor<Vec<u8>> {
    Cursor::new(Vec::new())
}

pub fn get_valid_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
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

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_merge_template_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates:
  "string":
    regex: "a"
nodes: {}"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_merge_node_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates: {}
nodes:
  "system":
    properties:
      "hostname":
        values:
          - template: string
        default:
         - literal: router
        deletable: true"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_new_node_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates: {}
nodes:
  "interfaces":
    subnodes: {}
    properties: {}"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_new_template_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates:
  "new":
    regex: "."
nodes: {}"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_invalid_regex_template_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates:
  "string":
    regex: "*"
nodes: {}"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_invalid_incl_incl_range_template_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates:
  "value":
    range:
      lower:
        inclusive: 1
      upper:
        inclusive: 0
nodes: {}"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_invalid_excl_incl_range_template_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates:
  "value":
    range:
      lower:
        exclusive: 1
      upper:
        inclusive: 0
nodes: {}"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_invalid_incl_excl_range_template_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates:
  "value":
    range:
      lower:
        inclusive: 0
      upper:
        exclusive: 0
nodes: {}"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_invalid_excl_excl_range_template_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates:
  "value":
    range:
      lower:
        exclusive: 0
      upper:
        exclusive: 0
nodes: {}"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_invalid_singlenode_prop_no_values_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates: {}
nodes:
  "invalid":
    properties:
      "invalid":
        values: []"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_invalid_singlenode_prop_value_template_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates: {}
nodes:
  "invalid":
    properties:
      "invalid":
        values:
          - template: nonexistent"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_invalid_singlenode_prop_value_range_schema() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates: {}
nodes:
  "invalid":
    properties:
      "invalid":
        values:
        - range:
            lower:
              inclusive: 2.0
            upper:
              inclusive: 1.0"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_invalid_singlenode_prop_multiple_defaults() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates: {}
nodes:
  "invalid":
    properties:
      "invalid":
        default:
          - literal: "1"
          - literal: "2"
        values:
          - literal: string"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_invalid_singlenode_prop_invalid_default_literal() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates: {}
nodes:
  "invalid":
    properties:
      "invalid":
        default:
          - literal: a
        values:
          - literal: b"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_invalid_singlenode_prop_invalid_default_template() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates:
  "digit":
    regex: "[0-9]"
nodes:
  "invalid":
    properties:
      "invalid":
        default:
          - literal: a
        values:
          - template: digit"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}

pub fn get_invalid_singlenode_prop_invalid_default_range() -> anyhow::Result<Schema> {
    let mut buf = buffer();
    let schema = r#"---
templates:
  "digit":
    regex: "[0-9]"
nodes:
  "invalid":
    properties:
      "invalid":
        default:
          - literal: "0.0"
        values:
          - range:
              lower:
                inclusive: 1.0
              upper:
                inclusive: 2.0"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(buf)
}
