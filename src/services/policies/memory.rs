use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

use async_lock::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use async_trait::async_trait;
use cedar_policy::{PolicySet, PolicySetError};
use log::{debug, info};

use crate::common;
use crate::schemas::policies::{Policy, PolicyUpdate};
use crate::services::policies::errors::PolicyStoreError;
use crate::services::policies::PolicyStore;

pub struct Policies(HashMap<String, cedar_policy::Policy>, PolicySet);

impl Policies {
    fn new() -> Self {
        Self {
            0: HashMap::new(),
            1: PolicySet::new(),
        }
    }

    #[allow(dead_code)]
    fn policy_map(&self) -> HashMap<String, cedar_policy::Policy> {
        self.0.clone()
    }

    fn policy_set(&self) -> PolicySet {
        self.1.clone()
    }

    fn update_policy_set(&mut self) {
        let mut policy_set = PolicySet::new();
        for policy in self.0.values() {
            policy_set.add(policy.clone()).unwrap();
        }
        self.1 = policy_set;
    }
}

pub struct MemoryPolicyStore {
    policies: RwLock<Policies>,
}

impl MemoryPolicyStore {
    pub fn new() -> Self {
        Self {
            policies: RwLock::new(Policies::new()),
        }
    }

    async fn read(&self) -> RwLockReadGuard<Policies> {
        debug!("Trying to acquire read lock on policies");
        self.policies.read().await
    }

    async fn write(&self) -> RwLockWriteGuard<Policies> {
        debug!("Trying to acquire write lock on policies");
        self.policies.write().await
    }
}

#[async_trait]
impl PolicyStore for MemoryPolicyStore {
    async fn policy_set(&self) -> PolicySet {
        let lock = self.read().await;
        lock.policy_set()
    }

    async fn get_policies(&self) -> Vec<Policy> {
        info!("Getting policies");
        let lock = self.read().await;
        Vec::from_iter(lock.0.values().cloned().map(|p| Policy::from(p)))
    }

    async fn get_policy(&self, id: &str) -> Result<Policy, Box<dyn Error>> {
        info!("Getting policy {}", id);
        let lock = self.read().await;
        let policy = lock.0.get(id);
        match policy {
            Some(p) => Ok(Policy::from(p.clone())),
            None => Err(PolicyStoreError::PolicyNotFoundError(id.to_owned()).into()),
        }
    }

    async fn create_policy(&self, policy: &Policy) -> Result<Policy, Box<dyn Error>> {
        info!("Creating policy {}", policy.id);
        let mut lock = self.write().await;
        let stored_policy = lock.0.get(&policy.id);
        match stored_policy {
            Some(_) => Err(PolicyStoreError::PolicySetError(PolicySetError::AlreadyDefined {
                id: cedar_policy::PolicyId::from_str(&policy.id).unwrap(),
            }).into()),
            None => {
                let policy: cedar_policy::Policy = match policy.borrow().try_into() {
                    Ok(p) => p,
                    Err(err) => return Err(err.into()),
                };
                let policy_id = policy.id().to_string();
                lock.0.insert(policy_id.clone(), policy);
                lock.update_policy_set();
                Ok(Policy::from(
                    lock.0.get(policy_id.as_str()).unwrap().clone(),
                ))
            }
        }
    }

    async fn update_policies(&self, policies: Vec<Policy>) -> Result<Vec<Policy>, Box<dyn Error>> {
        info!("Updating policies");
        let mut lock = self.write().await;
        let mut new_policies: HashMap<String, cedar_policy::Policy> = HashMap::new();
        for policy in policies {
            match new_policies.get(&policy.id) {
                Some(_) => return Err(PolicySetError::AlreadyDefined {
                    id: cedar_policy::PolicyId::from_str(&policy.id).unwrap(),
                }.into()),
                None => {
                    let policy: cedar_policy::Policy = match policy.borrow().try_into() {
                        Ok(p) => p,
                        Err(err) => return Err(err.into()),
                    };
                    new_policies.insert(policy.id().to_string(), policy)
                }
            };
        }
        lock.0 = new_policies;
        lock.update_policy_set();
        Ok(Vec::from_iter(
            lock.0.values().cloned().map(|p| Policy::from(p)),
        ))
    }

    async fn update_policy(
        &self,
        id: String,
        policy_update: PolicyUpdate,
    ) -> Result<Policy, Box<dyn Error>> {
        info!("Updating policy {}", id);
        let mut lock = self.write().await;
        let policy = Policy::from_policy_update(id.clone(), policy_update);
        let policy: cedar_policy::Policy = match policy.borrow().try_into() {
            Ok(p) => p,
            Err(err) => return Err(err.into()),
        };
        *lock
            .0
            .entry(String::from(id))
            .or_insert_with(|| policy.clone()) = policy.clone();
        lock.update_policy_set();
        Ok(Policy::from(policy))
    }

    async fn delete_policy(&self, id: &str) -> Result<Policy, Box<dyn Error>> {
        info!("Deleting policy {}", id);
        let mut lock = self.write().await;
        match lock.0.remove(id) {
            Some(policy) => {
                lock.update_policy_set();
                Ok(Policy::from(policy))
            }
            None => Err(common::EmptyError.into()),
        }
    }
}
