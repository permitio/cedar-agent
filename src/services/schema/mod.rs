use async_trait::async_trait;
use cedar_policy::Schema as CedarSchema;
use cedar_policy::SchemaError;

use crate::schemas::schema::Schema as InternalSchema;

pub mod memory;
pub mod load_from_file;

#[async_trait]
pub trait SchemaStore: Send + Sync {
    async fn schema(&self) -> CedarSchema;

    async fn schema_empty(&self) -> bool;

    async fn get_schema(&self) -> InternalSchema;
    async fn update_schema(
        &self,
        schema: InternalSchema
    ) -> Result<InternalSchema, SchemaError>;
    async fn delete_schema(&self);
}