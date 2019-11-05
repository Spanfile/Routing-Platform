#![allow(dead_code)]

use rp_schema::Schema;
use std::io::{Seek, SeekFrom, Write};
use tempfile::tempfile;

pub fn get_valid_schema() -> anyhow::Result<Schema> {
    let mut temp = tempfile()?;
    let schema = r#"---
templates:
 "string":
   regex: ".*"
nodes:
  "singlenode":
    properties:
      "query_default":
        default:
          - query:
              id: test
              command:
                cat: /test
        values:
          - template: string"#;

    write!(temp, "{}", schema)?;
    temp.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(&temp)
}

pub fn get_nonexistent_default_cat_query_schema() -> anyhow::Result<Schema> {
    let mut temp = tempfile()?;
    let schema = r#"---
templates: {}
nodes:
  "singlenode":
    properties:
      "query_default":
        default:
          - query:
              id: test
              command:
                cat: /nonexistent
        values:
          - literal: a"#;

    write!(temp, "{}", schema)?;
    temp.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(&temp)
}

pub fn get_invalid_default_cat_query_schema() -> anyhow::Result<Schema> {
    let mut temp = tempfile()?;
    let schema = r#"---
templates:
 "digit":
   regex: "[0-9]"
nodes:
  "singlenode":
    properties:
      "query_default":
        default:
          - query:
              id: test
              command:
                cat: /test
        values:
          - template: digit"#;

    write!(temp, "{}", schema)?;
    temp.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(&temp)
}

pub fn get_nonexistent_default_ls_query_schema() -> anyhow::Result<Schema> {
    let mut temp = tempfile()?;
    let schema = r#"---
templates:
 "digit":
   regex: "[0-9]"
nodes:
  "singlenode":
    properties:
      "query_default":
        default:
          - query:
              id: test
              command:
                ls: /nonexistent
        values:
          - template: digit"#;

    write!(temp, "{}", schema)?;
    temp.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(&temp)
}

pub fn get_invalid_default_ls_query_schema() -> anyhow::Result<Schema> {
    let mut temp = tempfile()?;
    let schema = r#"---
templates:
 "string":
   regex: ".*"
nodes:
  "singlenode":
    properties:
      "query_default":
        default:
          - query:
              id: test
              command:
                ls: /
        values:
          - template: string"#;

    write!(temp, "{}", schema)?;
    temp.seek(SeekFrom::Start(0))?;

    Schema::from_yaml_file(&temp)
}
