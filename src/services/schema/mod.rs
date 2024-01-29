use async_trait::async_trait;
use cedar_policy::Schema as CedarSchema;
use cedar_policy::SchemaError;

use crate::schemas::schema::Schema as InternalSchema;

pub mod memory;
pub mod load_from_file;

#[async_trait]
pub trait SchemaStore: Send + Sync {
    async fn get_cedar_schema(&self) -> Option<CedarSchema>;

    async fn get_internal_schema(&self) -> InternalSchema;
    async fn update_schema(
        &self,
        schema: InternalSchema
    ) -> Result<InternalSchema, SchemaError>;
    async fn delete_schema(&self);
}
