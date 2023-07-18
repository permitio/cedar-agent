use std::sync::Arc;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;

use crate::services::utils;

use cedar_agent::data::memory::MemoryDataStore;
use cedar_agent::data::load_from_file::load_entities_from_file;
use cedar_agent::DataStore;

#[tokio::test]
async fn memory_tests() {
    let store = MemoryDataStore::new();

    let entities = store.get_entities().await;
    assert_eq!(entities.len(), 0);
    let updated_entities = store.update_entities(utils::entities()).await.unwrap();
    assert_eq!(updated_entities.len(), 8);

    let error_entities = store.update_entities(utils::parse_error_entities()).await;
    assert!(error_entities.is_err());
    store.delete_entities().await;
    let entities = store.get_entities().await;
    assert_eq!(entities.len(), 0);
}

#[tokio::test]
async fn test_load_entities_from_file() {
    let temp_file_path = Arc::new(PathBuf::from("test_entities.json"));
    let test_data = r#"[{
        "attrs": {},
        "parents": [
          {
            "id": "Admin",
            "type": "Role"
          }
        ],
        "uid": {
          "id": "admin.1@domain.com",
          "type": "User"
        }
    }]"#;
    let mut temp_file = File::create(&*temp_file_path.clone()).unwrap();
    temp_file.write_all(test_data.as_bytes()).unwrap();

    let entities = load_entities_from_file(temp_file_path.clone().to_path_buf()).await.unwrap();
    assert_eq!(entities.len(), 1);

    std::fs::remove_file(temp_file_path.clone().to_path_buf()).unwrap();
}