use rocket::{delete, get, put, State};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::authn::ApiKey;
use crate::errors::response::AgentError;
use cedar_policy::Schema as CedarSchema;
use log::error;
use crate::schemas::schema::Schema as InternalSchema;
use crate::services::{schema::SchemaStore, policies::PolicyStore, data::DataStore};

#[openapi]
#[get("/schema")]
pub async fn get_schema(
    _auth: ApiKey,
    schema_store: &State<Box<dyn SchemaStore>>
) -> Result<Json<InternalSchema>, AgentError> {
    Ok(Json::from(schema_store.get_internal_schema().await))
}

#[openapi]
#[put("/schema", format = "json", data = "<schema>")]
pub async fn update_schema(
    _auth: ApiKey,
    schema_store: &State<Box<dyn SchemaStore>>,
    policy_store: &State<Box<dyn PolicyStore>>,
    data_store: &State<Box<dyn DataStore>>,
    schema: Json<InternalSchema>
) -> Result<Json<InternalSchema>, AgentError> {
    let cedar_schema: CedarSchema = match schema.clone().into_inner().try_into() {
        Ok(schema) => schema,
        Err(err) => return Err(AgentError::BadRequest {
            reason: err.to_string(),
        })
    };

    let current_policies = policy_store.get_policies().await;
    match policy_store.update_policies(current_policies, Some(cedar_schema.clone())).await {
        Ok(_) => {},
        Err(err) => return Err(AgentError::BadRequest {
            reason: format!("Existing policies invalid with the new schema: {}", err.to_string()),
        })
    }

    let current_entities = data_store.get_entities().await;
    match data_store.update_entities(current_entities, Some(cedar_schema)).await {
        Ok(_) => {},
        Err(err) => return Err(AgentError::BadRequest {
            reason: format!("Existing entities invalid with the new schema: {}", err.to_string()),
        })
    }

    match schema_store.update_schema(schema.into_inner()).await {
        Ok(schema) => Ok(Json::from(schema)),
        Err(err) => return Err(AgentError::BadRequest {
            reason: err.to_string(),
        })
    }
}

#[openapi]
#[delete("/schema")]
pub async fn delete_schema(
    _auth: ApiKey,
    schema_store: &State<Box<dyn SchemaStore>>
) -> Result<status::NoContent, AgentError> {
    schema_store.delete_schema().await;
    Ok(status::NoContent)
}
