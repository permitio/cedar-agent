use cedar_policy::Authorizer;

use log::info;

use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_okapi::openapi;

use crate::authn::ApiKey;
use crate::errors::response::AgentError;
use crate::schemas::authorization::{AuthorizationAnswer, AuthorizationCall, AuthorizationRequest};
use crate::{DataStore, PolicyStore};

#[openapi]
#[post("/is_authorized", format = "json", data = "<authorization_call>")]
pub async fn is_authorized(
    _auth: ApiKey,
    policy_store: &State<Box<dyn PolicyStore>>,
    data_store: &State<Box<dyn DataStore>>,
    authorizer: &State<Authorizer>,
    authorization_call: Json<AuthorizationCall>,
) -> Result<Json<AuthorizationAnswer>, AgentError> {
    let policies = policy_store.policy_set().await;
    let query: AuthorizationRequest = match authorization_call.into_inner().try_into() {
        Ok(query) => query,
        Err(err) => {
            return Err(AgentError::BadRequest {
                reason: err.to_string(),
            })
        }
    };

    // Temporary solution to override fetching entities from the datastore by directly passing it to the REST body.
    // Eventually this logic will be replaced in favor of performing live patch updates
    let stored_entities = data_store.entities().await;
    let (request, entities) = match query.get_request_entities(stored_entities) {
        Ok(result) => result,
        Err(err)=> {
            return Err(AgentError::BadRequest {
                reason: err.to_string(),
            })
        }
    };

    info!("Querying cedar using {:?}", &request);
    let answer = authorizer.is_authorized(&request, &policies, &entities);
    Ok(Json::from(AuthorizationAnswer::from(answer)))
}
