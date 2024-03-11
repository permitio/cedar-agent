use std::error::Error;

use async_trait::async_trait;
use cedar_policy::{PolicySet, Schema};

use crate::schemas::policies::{Policy, PolicyUpdate};

pub(crate) mod errors;
pub mod memory;
pub mod load_from_file;

#[async_trait]
pub trait PolicyStore: Send + Sync {
    async fn policy_set(&self) -> PolicySet;
    async fn get_policies(&self) -> Vec<Policy>;
    async fn get_policy(&self, id: &str) -> Result<Policy, Box<dyn Error>>;
    async fn create_policy(
        &self,
        policy: &Policy,
        schema: Option<Schema>
    ) -> Result<Policy, Box<dyn Error>>;
    async fn update_policies(
        &self,
        policies: Vec<Policy>,
        schema: Option<Schema>
    ) -> Result<Vec<Policy>, Box<dyn Error>>;
    async fn update_policy(
        &self,
        id: String,
        policy: PolicyUpdate,
        schema: Option<Schema>,
    ) -> Result<Policy, Box<dyn Error>>;
    async fn delete_policy(&self, id: &str) -> Result<Policy, Box<dyn Error>>;
}
