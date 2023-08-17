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

    let (request, entities) = &query.get_request_entities();    

    /// temporary solution for now. Eventually this logic will be replaced in favor of performing live patch updates  
    let request_entities: cedar_policy::Entities = if *entities == cedar_policy::Entities::empty() { data_store.entities().await } else { entities.clone() };
    
    info!("Querying cedar using {:?}", &request);
    let answer = authorizer.is_authorized(&request, &policies, &request_entities);
    Ok(Json::from(AuthorizationAnswer::from(answer)))
}
