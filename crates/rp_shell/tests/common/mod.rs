use rp_config::Config;
use rp_schema::Schema;
use std::{
    io::{Read, Seek, SeekFrom, Write},
    rc::Rc,
};
use tempfile::tempfile;

pub fn get_schema_and_config() -> anyhow::Result<(Rc<Schema>, Config)> {
    let mut temp = tempfile()?;
    let schema = r#"---
templates:
  "digit":
    regex: "[0-9]"
nodes:
  "multinode":
    source:
      template: digit
    node:
      properties:
        "simple":
          values:
          - literal: a
  "singlenode":
    subnodes:
      "subnode":
        properties:
          "simple":
            values:
            - template: digit
    properties:
      "query_default":
        default:
          - query:
              id: test
              command:
                cat: /test
        values:
          - literal: a"#;

    write!(temp, "{}", schema)?;
    temp.seek(SeekFrom::Start(0))?;

    let mut schema = Schema::from_yaml_file(&temp)?;
    schema.build_regex_cache()?;

    let mut file = tempfile()?;
    schema.to_binary_file(&mut file)?;

    file.seek(SeekFrom::Start(0))?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let schema = Rc::new(Schema::from_binary(&buf)?);

    Ok((
        Rc::clone(&schema),
        Config::from_schema(Rc::downgrade(&schema))?,
    ))
}
