use rocket::serde::json::serde_json::from_str;

use cedar_agent::schemas::data::Entities;
use cedar_agent::schemas::policies::Policy;
use cedar_agent::schemas::schema::Schema;

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
    let id = id.unwrap_or_else(|| "test".to_string());
    Policy {
        id,
        content: "permit(principal,action,resource);".to_string(),
    }
}

pub(crate) fn approve_admin_policy(id: Option<String>) -> Policy {
    let id = id.unwrap_or_else(|| "test".to_string());
    Policy {
        id,
        content: "permit(principal == User::\"admin@domain.com\",action,resource);".to_string(),
    }
}

pub(crate) fn schema_valid_policy(id: Option<String>) -> Policy {
    let id = id.unwrap_or_else(|| "test".to_string());
    Policy {
        id,
        content: "permit(principal in Role::\"Editor\",action,resource == ResourceType::\"document\");".to_string(),
    }
}

pub(crate) fn schema_invalid_policy(id: Option<String>) -> Policy {
    let id = id.unwrap_or_else(|| "test".to_string());
    Policy {
        id,
        content: "permit(principal in Role::\"Editor\",action,resource == Document::\"document\");".to_string(),
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

pub(crate) fn schema() -> Schema {
    let schema_json = r#"
{
      "": {
        "entityTypes": {
          "User": {
            "shape": {
              "type": "Record",
              "attributes": {
                "confidenceScore": {
                  "type": "Extension",
                  "name": "decimal"
                },
                "department": {
                  "type": "String"
                },
                "homeIp": {
                  "type": "Extension",
                  "name": "ipaddr"
                },
                "jobLevel": {
                  "type": "Long"
                }
              }
            },
            "memberOfTypes": [
              "Role"
            ]
          },
          "Role": {
            "shape": {
              "type": "Record",
              "attributes": {}
            }
          },
          "ResourceType": {
            "shape": {
              "type": "Record",
              "attributes": {}
            }
          }
        },
        "actions": {
          "document:get": {
            "appliesTo": {
              "principalTypes": [
                "User",
                "Role"
              ],
              "resourceTypes": [
                "ResourceType"
              ]
            }
          },
          "document:create": {
            "memberOf": [
              {
                "id": "document:update"
              }
            ],
            "appliesTo": {
              "principalTypes": [
                "User",
                "Role"
              ],
              "resourceTypes": [
                "ResourceType"
              ]
            }
          },
          "document:delete": {
            "appliesTo": {
              "principalTypes": [
                "User",
                "Role"
              ],
              "resourceTypes": [
                "ResourceType"
              ]
            }
          },
          "document:update": {
            "memberOf": [
              {
                "id": "document:delete"
              }
            ],
            "appliesTo": {
              "principalTypes": [
                "User",
                "Role"
              ],
              "resourceTypes": [
                "ResourceType"
              ]
            }
          },
          "document:list": {
            "memberOf": [
              {
                "id": "document:get"
              }
            ],
            "appliesTo": {
              "principalTypes": [
                "User",
                "Role"
              ],
              "resourceTypes": [
                "ResourceType"
              ]
            }
          }
        }
      }
    }
    "#;
    from_str(schema_json).unwrap()
}

pub(crate) fn parse_error_schema() -> Schema {
    let schema_json = r#"
    {
      "namespace": "error"
    }
    "#;
    from_str(schema_json).unwrap()
}
