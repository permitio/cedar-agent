use rocket::{delete, get, put, State};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::authn::ApiKey;
use crate::errors::response::AgentError;
use crate::schemas::schema::Schema;
use crate::services::schema::SchemaStore;

#[openapi]
#[get("/schema")]
pub async fn get_schema(
    _auth: ApiKey,
    schema_store: &State<Box<dyn SchemaStore>>
) -> Result<Json<Schema>, AgentError> {
    Ok(Json::from(schema_store.get_schema().await))
}

#[openapi]
#[put("/schema", format = "json", data = "<schema>")]
pub async fn update_schema(
    _auth: ApiKey,
    schema_store: &State<Box<dyn SchemaStore>>,
    schema: Json<Schema>
) -> Result<Json<Schema>, AgentError> {
    match schema_store.update_schema(schema.into_inner()).await {
        Ok(schema) => Ok(Json::from(schema)),
        Err(err) => Err(AgentError::BadRequest {
            reason: err.to_string(),
        }),
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