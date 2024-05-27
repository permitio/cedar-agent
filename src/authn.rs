use rocket::request::{FromRequest, Outcome};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi;
use rocket_okapi::okapi::openapi3::{
    Object, Responses, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};

use crate::config::Config;

const AUTHENTICATION_HEADER: &'static str = "Authorization";

pub struct ApiKey(Option<String>);

impl ApiKey {
    fn validate_matching_header(&self, request: &rocket::Request) -> bool {
        let required_token = self.0.clone();
        if required_token.is_none() {
            return true;
        }
        let token = request.headers().get_one(AUTHENTICATION_HEADER);
        match token {
            Some(token) => token == required_token.unwrap(),
            None => false,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request
            .rocket()
            .state::<Config>()
            .map(|my_config| ApiKey(my_config.authentication.clone()));
        match token {
            Some(token) => {
                if token.validate_matching_header(request) {
                    Outcome::Success(token)
                } else {
                    Outcome::Error((rocket::http::Status::Unauthorized,()))
                }
            }
            None => Outcome::Success(ApiKey(None)),
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for ApiKey {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some(
                r#"Optional API key to access, 
            used if the agent was started with authentication configuration."#
                    .to_owned(),
            ),
            // Setup data requirements.
            // This can be part of the `header`, `query` or `cookie`.
            // In this case the header `authorization:` needs to be set.
            data: SecuritySchemeData::ApiKey {
                name: "authorization".to_owned(),
                location: "header".to_owned(),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        // This can change between routes.
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("ApiKeyAuth".to_owned(), Vec::new());
        // These vvvvvvv-----^^^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "ApiKeyAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }

    // Optionally add responses
    // Also see `main.rs` part of this.
    fn get_responses(gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        use rocket_okapi::okapi::openapi3::RefOr;
        // Can switch between to the but both are checked if they compile correctly
        Ok(Responses {
            // Recommended and most strait forward.
            // And easy to add or remove new responses.
            responses: okapi::map! {
                "400".to_owned() => RefOr::Object(crate::errors::schemas::bad_request_response(gen)),
                "401".to_owned() => RefOr::Object(crate::errors::schemas::unauthorized_response(gen)),
            },
            ..Default::default()
        })
    }
}
