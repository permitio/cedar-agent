use std::borrow::Borrow;

use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::serde_json;
use rocket::{response, Request, Response};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::{okapi, OpenApiError};
use serde::Serialize;
use thiserror::Error;

use schemas::{bad_request_response, unauthorized_response};

use crate::errors::schemas;

/// Error messages returned to user
#[derive(Debug, Serialize, JsonSchema)]
pub struct ErrorResponse {
    /// The title of the error message
    pub reason: String,
    /// The description of the error
    pub description: String,
    // HTTP Status Code returned
    pub code: u16,
}

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        // Convert object to json
        let body = serde_json::to_string(&self).unwrap();
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(ContentType::JSON)
            .status(Status::new(self.code))
            .ok()
    }
}

#[derive(Debug, Error)]
pub enum AgentError {
    #[error(" {} with the given id({}) not found", object, id)]
    NotFound { object: &'static str, id: String },
    #[error("{} with the given id({}) already exists", object, id)]
    Duplicate { object: &'static str, id: String },
    #[error(
        "The content in the request does not match the specifications: {}",
        reason
    )]
    BadRequest { reason: String },
}

impl AgentError {
    fn status(&self) -> Status {
        use self::AgentError::*;
        match self {
            NotFound { object: _, id: _ } => Status::NotFound,
            Duplicate { object: _, id: _ } => Status::Conflict,
            BadRequest { reason: _ } => Status::BadRequest,
        }
    }

    fn title(&self) -> String {
        let status = self.status();
        // use if else if because
        // the traits must be derived, manual `impl`s are not sufficient
        // compile error is raised when using match
        if status == Status::BadRequest {
            "You have malformed a bad request".to_owned()
        } else if status == Status::Unauthorized {
            "You are not authorized to perform this action".to_owned()
        } else if status == Status::NotFound {
            "The requested resource was not found".to_owned()
        } else if status == Status::Conflict {
            "The requested resource already exists".to_owned()
        } else if status.code >= 400 && status.code < 500 {
            "An unexpected client error has occurred".to_owned()
        } else {
            "An unexpected server error has occurred".to_owned()
        }
    }

    fn message(&self) -> String {
        format!("{self}")
    }
}

impl<'r> Responder<'r, 'static> for AgentError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let res = ErrorResponse {
            code: self.status().code,
            reason: self.title(),
            description: self.message(),
        };
        // Convert object to json
        let body = serde_json::to_string(res.borrow()).unwrap();
        Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(ContentType::JSON)
            .status(Status::new(res.code))
            .ok()
    }
}

impl OpenApiResponderInner for AgentError {
    fn responses(gen: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use okapi::openapi3::RefOr;
        Ok(Responses {
            responses: okapi::map! {
                "400".to_owned() => RefOr::Object(bad_request_response(gen)),
                "401".to_owned() => RefOr::Object(unauthorized_response(gen)),
            },
            ..Default::default()
        })
    }
}
