use super::Schema;

pub trait Validate {
    fn validate(&self, schema: &Schema) -> anyhow::Result<()>;
}
