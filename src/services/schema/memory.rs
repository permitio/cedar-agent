use std::str::FromStr;

use async_lock::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use async_trait::async_trait;
use cedar_policy::Schema as CedarSchema;
use cedar_policy::SchemaError;
use log::{debug, error, info};

use crate::schemas::schema::Schema as InternalSchema;
use crate::services::schema::SchemaStore;

pub struct Schema(CedarSchema, InternalSchema);

impl Schema {
    fn empty() -> Self {
        Self {
            0: CedarSchema::from_str("{}").unwrap(),
            1: InternalSchema::empty()
        }
    }

    fn cedar_schema(&self) -> CedarSchema {
        self.0.clone()
    }

    fn internal_schema(&self) -> InternalSchema {
        self.1.clone()
    }

    fn new(cedar_schema: CedarSchema, internal_schema: InternalSchema) -> Self {
        Self {
            0: cedar_schema,
            1: internal_schema
        }
    }
}

pub struct MemorySchemaStore {
    schema: RwLock<Schema>
}

impl MemorySchemaStore {
    pub fn new() -> Self {
        Self {
            schema: RwLock::new(Schema::empty())
        }
    }

    async fn read(&self) -> RwLockReadGuard<Schema> {
        debug!("Trying to acquire read lock on the schema");
        self.schema.read().await
    }

    async fn write(&self) -> RwLockWriteGuard<Schema> {
        debug!("Trying to acquire write lock on the schema");
        self.schema.write().await
    }
}

#[async_trait]
impl SchemaStore for MemorySchemaStore {
    async fn get_cedar_schema(&self) -> Option<CedarSchema> {
        let lock = self.read().await;
        if lock.internal_schema().is_empty() {
            None
        } else {
            Some(lock.cedar_schema())
        }
    }

    async fn get_internal_schema(&self) -> InternalSchema {
        info!("Getting stored schema");
        let lock = self.read().await;
        lock.internal_schema()
    }

    async fn update_schema(
        &self,
        schema: InternalSchema
    ) -> Result<InternalSchema, SchemaError> {
        info!("Updating stored schema");
        let mut lock = self.write().await;
        let internal_schema: InternalSchema = schema.clone();
        let cedar_schema: CedarSchema = match schema.try_into() {
            Ok(schema) => schema,
            Err(err) => {
                error!("Failed to parse schema");
                return Err(err);
            }
        };
        *lock = Schema::new(cedar_schema, internal_schema.clone());
        Ok(internal_schema)
    }

    async fn delete_schema(&self) {
        info!("Deleting stored schema");
        let mut lock = self.write().await;
        *lock = Schema::empty();
    }
}
