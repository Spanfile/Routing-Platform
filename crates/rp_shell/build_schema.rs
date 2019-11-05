use rp_schema::Schema;
use std::{env, fs::File, path::Path};

fn main() -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let pwd = env::current_dir()?;

    let schema_path = Path::new(&pwd).join("schema.yml");
    let dest_path = Path::new(&out_dir).join("schema");

    let schema_file = File::open(&schema_path)?;
    let mut schema = Schema::from_yaml_file(&schema_file)?;
    schema.validate()?;
    schema.build_regex_cache()?;

    let mut schema_dest = File::create(dest_path)?;
    schema.to_binary_file(&mut schema_dest)?;

    Ok(())
}
