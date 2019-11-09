use rp_config::Config;
use rp_schema::Schema;
use std::{
    io::{Cursor, Read, Seek, SeekFrom, Write},
    rc::Rc,
};

fn buffer() -> Cursor<Vec<u8>> {
    Cursor::new(Vec::new())
}

pub fn get_schema_and_config() -> anyhow::Result<(Rc<Schema>, Config)> {
    let mut buf = buffer();
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
      "subnode": {}
    properties:
      "simple":
        values:
        - template: digit
      "default":
        default:
        - literal: "0"
        values:
        - template: digit
      "multiple":
        multiple: true
        default:
        - literal: "0"
        - literal: "1"
        values:
        - template: digit
      "query_default":
        default:
          - query:
              id: test
              command:
                cat: /test
        values:
          - literal: a"#;

    write!(buf, "{}", schema)?;
    buf.seek(SeekFrom::Start(0))?;

    let mut schema = Schema::from_yaml_file(buf)?;
    schema.build_regex_cache()?;

    let mut buf = buffer();
    schema.to_binary_file(&mut buf)?;

    buf.seek(SeekFrom::Start(0))?;

    let mut bytes = Vec::new();
    buf.read_to_end(&mut bytes)?;
    let schema = Rc::new(Schema::from_binary(&bytes)?);

    Ok((
        Rc::clone(&schema),
        Config::from_schema(Rc::downgrade(&schema))?,
    ))
}
