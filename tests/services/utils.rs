use rocket::serde::json::serde_json::from_str;

use cedar_agent::schemas::data::Entities;
use cedar_agent::schemas::policies::Policy;

pub(crate) fn split_content(in_string: &str) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, ':');
    let id = splitter.next().unwrap();
    let content = splitter.next().unwrap();
    (id, content)
}

pub(crate) fn parse_error_policy() -> Policy {
    Policy {
        id: "error".to_string(),
        content: "error".to_string(),
    }
}

pub(crate) fn approve_all_policy(id: Option<String>) -> Policy {
    let id = match id {
        Some(id) => id,
        None => "test".to_string(),
    };
    Policy {
        id: id,
        content: "permit(principal,action,resource);".to_string(),
    }
}

pub(crate) fn approve_admin_policy(id: Option<String>) -> Policy {
    let id = match id {
        Some(id) => id,
        None => "test".to_string(),
    };
    Policy {
        id: id,
        content: "permit(principal == User::\"admin@domain.com\",action,resource);".to_string(),
    }
}

pub(crate) fn entities() -> Entities {
    let entities_json = r#"
    [
      {
        "attrs": {
          "confidenceScore": {
            "__extn": {
              "arg": "33.57",
              "fn": "decimal"
            }
          },
          "department": "HardwareEngineering",
          "homeIp": {
            "__extn": {
              "arg": "222.222.222.7",
              "fn": "ip"
            }
          },
          "jobLevel": 5
        },
        "parents": [
          {
            "id": "Editor",
            "type": "Role"
          }
        ],
        "uid": {
          "id": "editor-1@domain.com",
          "type": "User"
        }
      },
      {
        "attrs": {},
        "parents": [],
        "uid": {
          "id": "document:delete",
          "type": "Action"
        }
      },
      {
        "attrs": {},
        "parents": [
          {
            "id": "document:update",
            "type": "Action"
          }
        ],
        "uid": {
          "id": "document:create",
          "type": "Action"
        }
      },
      {
        "attrs": {},
        "parents": [],
        "uid": {
          "id": "document",
          "type": "ResourceType"
        }
      },
      {
        "attrs": {},
        "parents": [
          {
            "id": "document:delete",
            "type": "Action"
          }
        ],
        "uid": {
          "id": "document:update",
          "type": "Action"
        }
      },
      {
        "attrs": {},
        "parents": [
          {
            "id": "document:get",
            "type": "Action"
          }
        ],
        "uid": {
          "id": "document:list",
          "type": "Action"
        }
      },
      {
        "attrs": {},
        "parents": [],
        "uid": {
          "id": "document:get",
          "type": "Action"
        }
      },
      {
        "attrs": {},
        "parents": [],
        "uid": {
          "id": "Editor",
          "type": "Role"
        }
      }
    ]
    "#;
    from_str(entities_json).unwrap()
}

pub(crate) fn parse_error_entities() -> Entities {
    let entities_json = r#"
    [
      {
        "id": "error"
      }
    ]
    "#;
    from_str(entities_json).unwrap()
}
