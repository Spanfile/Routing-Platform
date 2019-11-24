use rp_schema::Schema;
use std::{env, fs::File, path::Path};

fn main() -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let pwd = env::current_dir()?;

    let schema_path = Path::new(&pwd).join("schema.yml");
    let debug_schema_path = Path::new(&pwd).join("debug_schema.yml");

    let dest_path = Path::new(&out_dir).join("schema");
    let debug_dest_path = Path::new(&out_dir).join("debug_schema");

    let schema_file = File::open(&schema_path)?;
    let mut schema = Schema::from_yaml_file(&schema_file)?;
    schema.validate()?;
    schema.build_regex_cache()?;

    let debug_schema_file = File::open(&debug_schema_path)?;
    let mut debug_schema = Schema::from_yaml_file(&debug_schema_file)?;
    debug_schema.validate()?;

    let mut schema_dest = File::create(dest_path)?;
    let mut debug_schema_dest = File::create(debug_dest_path)?;
    schema.to_binary_file(&mut schema_dest)?;
    debug_schema.to_binary_file(&mut debug_schema_dest)?;

    Ok(())
}
