#![allow(dead_code)]

use rp_config::Config;
use rp_schema::Schema;
use std::{
    io::{Cursor, Read, Seek, SeekFrom, Write},
    rc::Rc,
};

fn buffer() -> Cursor<Vec<u8>> {
    Cursor::new(Vec::new())
}

pub fn get_valid_config() -> anyhow::Result<Config> {
    let mut schema = get_valid_schema()?;
    schema.build_regex_cache()?;

    let mut buf = buffer();
    schema.to_binary_file(&mut buf)?;

    buf.seek(SeekFrom::Start(0))?;

    let mut bytes = Vec::new();
    buf.read_to_end(&mut bytes)?;
    let schema = Rc::new(Schema::from_binary(&bytes)?);

    Config::from_schema(Rc::downgrade(&schema))
}

pub fn get_valid_schema() -> anyhow::Result<Schema> {
    let mut temp = buffer();
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

    Schema::from_yaml_file(temp)
}

pub fn get_nonexistent_default_cat_query_schema() -> anyhow::Result<Schema> {
    let mut temp = buffer();
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

    Schema::from_yaml_file(temp)
}

pub fn get_invalid_default_cat_query_schema() -> anyhow::Result<Schema> {
    let mut temp = buffer();
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

    Schema::from_yaml_file(temp)
}

pub fn get_nonexistent_default_ls_query_schema() -> anyhow::Result<Schema> {
    let mut temp = buffer();
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

    Schema::from_yaml_file(temp)
}

pub fn get_invalid_default_ls_query_schema() -> anyhow::Result<Schema> {
    let mut temp = buffer();
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

    Schema::from_yaml_file(temp)
}
