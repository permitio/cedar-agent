use std::path::PathBuf;

use cedar_agent::schema::load_from_file::load_schema_from_file;
use cedar_agent::schema::memory::MemorySchemaStore;
use cedar_agent::policies::memory::MemoryPolicyStore;
use cedar_agent::data::memory::MemoryDataStore;
use cedar_agent::{SchemaStore, PolicyStore, DataStore};

use crate::services::utils;

#[tokio::test]
async fn memory_tests() {
    let store = MemorySchemaStore::new();

    let schema = store.get_internal_schema().await;
    assert!(schema.is_empty());
    let updated_schema = store.update_schema(utils::schema()).await;
    assert!(!updated_schema.is_err());
    assert!(!updated_schema.unwrap().is_empty());

    let error_schema = store.update_schema(utils::parse_error_schema()).await;
    assert!(error_schema.is_err());
    store.delete_schema().await;
    let schema = store.get_internal_schema().await;
    assert!(schema.is_empty());
}

#[tokio::test]
async fn test_load_schema_from_file() {
    let schema = load_schema_from_file(PathBuf::from("./examples/schema.json"))
        .await
        .unwrap();
    assert!(!schema.is_empty());
}

#[tokio::test]
async fn test_validate_policy() {
    let policy_store = MemoryPolicyStore::new();
    let schema_store = MemorySchemaStore::new();
    schema_store.update_schema(utils::schema()).await.unwrap();

    let valid_policies = policy_store
        .update_policies(
            vec![utils::schema_valid_policy(Some("valid".to_string()))],
            schema_store.get_cedar_schema().await
        ).await;
    assert!(!valid_policies.is_err());

    let policies = valid_policies.unwrap();
    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].id, "valid");

    let invalid_policies = policy_store
        .update_policies(
            vec![utils::schema_invalid_policy(Some("invalid".to_string()))],
            schema_store.get_cedar_schema().await
        ).await;
    assert!(invalid_policies.is_err());
}

#[tokio::test]
async fn test_validate_entities() {
    let data_store = MemoryDataStore::new();
    let schema_store = MemorySchemaStore::new();
    schema_store.update_schema(utils::schema()).await.unwrap();

    let valid_entities = data_store
        .update_entities(
            utils::entities(),
            schema_store.get_cedar_schema().await
        ).await;
    assert!(!valid_entities.is_err());
    assert_eq!(valid_entities.unwrap().len(), 8);

    let invalid_entities = data_store
        .update_entities(
            utils::parse_error_entities(),
            schema_store.get_cedar_schema().await
        ).await;
    assert!(invalid_entities.is_err());
}
