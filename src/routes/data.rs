use rocket::response::status;

use rocket::serde::json::Json;
use rocket::{delete, get, put, State};
use rocket_okapi::openapi;

use crate::authn::ApiKey;
use crate::errors::response::AgentError;
use crate::schemas::data as schemas;
use crate::DataStore;

#[openapi]
#[get("/data")]
pub async fn get_entities(
    _auth: ApiKey,
    data_store: &State<Box<dyn DataStore>>,
) -> Result<Json<schemas::Entities>, AgentError> {
    Ok(Json::from(data_store.get_entities().await))
}

#[openapi]
#[put("/data", format = "json", data = "<entities>")]
pub async fn update_entities(
    _auth: ApiKey,
    data_store: &State<Box<dyn DataStore>>,
    entities: Json<schemas::Entities>,
) -> Result<Json<schemas::Entities>, AgentError> {
    match data_store.update_entities(entities.into_inner()).await {
        Ok(entities) => Ok(Json::from(entities)),
        Err(err) => Err(AgentError::BadRequest {
            reason: err.to_string(),
        }),
    }
}

#[openapi]
#[delete("/data")]
pub async fn delete_entities(
    _auth: ApiKey,
    data_store: &State<Box<dyn DataStore>>,
) -> Result<status::NoContent, AgentError> {
    data_store.delete_entities().await;
    Ok(status::NoContent)
}
