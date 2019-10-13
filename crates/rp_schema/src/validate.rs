use super::Schema;
use crate::error;

pub trait Validate {
    fn validate(&self, schema: &Schema) -> error::Result<()>;
}
