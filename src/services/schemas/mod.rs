use async_trait::async_trait;
use cedar_policy::Schema as CedarSchema;
use cedar_policy::SchemaError;

use crate::schemas::schemas::Schema as InternalSchema;

pub mod schemas;

#[async_trait]
pub trait SchemaStore: Send + Sync {
    async fn schema(&self) -> CedarSchema;
    async fn get_schema(&self) -> InternalSchema;
    async fn update_schema(
        &self,
        schema: InternalSchema
    ) -> Result<InternalSchema, SchemaError>;
    async fn delete_schema(&self);
}