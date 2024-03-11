use std::path::PathBuf;

use crate::services::utils;
use std::error::Error;

use cedar_agent::data::load_from_file::load_entities_from_file;
use cedar_agent::data::memory::MemoryDataStore;
use cedar_agent::schemas::authorization::AuthorizationCall;
use cedar_agent::schemas::authorization::AuthorizationRequest;
use cedar_agent::DataStore;
use cedar_policy::Entities;

#[tokio::test]
async fn memory_tests() {
    let store = MemoryDataStore::new();

    let entities = store.get_entities().await;
    assert_eq!(entities.len(), 0);
    let updated_entities = store.update_entities(utils::entities(), None).await.unwrap();
    assert_eq!(updated_entities.len(), 8);

    let error_entities = store.update_entities(utils::parse_error_entities(), None).await;
    assert!(error_entities.is_err());
    store.delete_entities().await;
    let entities = store.get_entities().await;
    assert_eq!(entities.len(), 0);
}

#[tokio::test]
async fn test_load_entities_from_file() {
    let entities = load_entities_from_file(PathBuf::from("./examples/data.json"))
        .await
        .unwrap();
    assert_eq!(entities.len(), 12);
}

#[tokio::test]
async fn test_load_empty_entities_from_authz_call() {

    let entities: String = String::from("[]");

    let query = make_authz_call(entities);

    match query {
        Ok(req) => assert_eq!(req.get_entities().unwrap(), Entities::empty()),
        _ => assert!(false),
    };
}

#[tokio::test]
async fn test_load_no_entities_from_authz_call() {

    let query = make_authz_call_no_entities();

    match query {
        Ok(req) => assert_eq!(req.get_entities(), None),
        _ => assert!(false),
    };
}


#[tokio::test]
async fn test_load_entities_from_authz_call() {

    let entities: String = r#"
    [
        {
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
        },
        {
            "attrs": {},
            "parents": [],
            "uid": {
                "id": "delete",
                "type": "Action"
            }
        }
    ]
    "#
    .to_string();

    let query = make_authz_call(entities);

    match query {
        Ok(req) => {
            assert_ne!(req.get_entities(), None);
        },
        _ => assert!(false)
    };
}

#[tokio::test]
async fn test_combine_entities_with_additional_entities(){
    let stored_entities: String = r#"
    [
        {
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
        },
        {
            "attrs": {},
            "parents": [],
            "uid": {
                "id": "delete",
                "type": "Action"
            }
        }
    ]
    "#
    .to_string();

    let additional_entities: String = r#"
    [
        {
            "attrs": {},
            "parents": [],
            "uid": {
                "id": "Admin",
                "type": "Role"
            }
        }
    ]
    "#.to_string();

    let expected_result: String = r#"
    [
        {
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
        },
        {
            "attrs": {},
            "parents": [],
            "uid": {
                "id": "delete",
                "type": "Action"
            }
        },
        {
            "attrs": {},
            "parents": [],
            "uid": {
                "id": "Admin",
                "type": "Role"
            }
        }
    ]
    "#.to_string();

    let query = make_authz_call_with_additional_entities(additional_entities);

    match query {
        Ok(req) => {
            match req.get_request_entities(Entities::from_json_str(&stored_entities, None).unwrap()) {
                Ok((_request, entities)) => {
                    assert_eq!(entities, Entities::from_json_str(&expected_result, None).unwrap())
                },
                _ => assert!(false)
            };
        },
        _ => assert!(false)
    };
}

fn make_authz_call_no_entities() -> Result<AuthorizationRequest, Box<dyn Error>> {
    let principal: Option<String> = Some("User::\"Test\"".to_string());
    let action: Option<String> = Some("Action::\"Delete\"".to_string());
    let resource: Option<String> = Some("Document::\"cedar-agent.pdf\"".to_string());

    let authorization_call = AuthorizationCall::new(
        principal,
        action,
        resource,
        None,
        None,
        None,
        None,
    );
    return authorization_call.try_into();
}

fn make_authz_call(entities: String) -> Result<AuthorizationRequest, Box<dyn Error>> {
    let principal: Option<String> = Some("User::\"Test\"".to_string());
    let action: Option<String> = Some("Action::\"Delete\"".to_string());
    let resource: Option<String> = Some("Document::\"cedar-agent.pdf\"".to_string());

    let authorization_call = AuthorizationCall::new(
        principal,
        action,
        resource,
        None,
        rocket::serde::json::from_str(&entities).unwrap(),
        None,
        None,
    );
    return authorization_call.try_into();
}

fn make_authz_call_with_additional_entities(
    additional_entities: String
) -> Result<AuthorizationRequest, Box<dyn Error>> {
    let principal: Option<String> = Some("User::\"Test\"".to_string());
    let action: Option<String> = Some("Action::\"Delete\"".to_string());
    let resource: Option<String> = Some("Document::\"cedar-agent.pdf\"".to_string());

    let authorization_call = AuthorizationCall::new(
        principal,
        action,
        resource,
        None,
        None,
        rocket::serde::json::from_str(&additional_entities).unwrap(),
        None,
    );
    return authorization_call.try_into();
}
