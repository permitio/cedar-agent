use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

use cedar_policy::{Context, EntityUid, EvaluationError, Request, Response};
use cedar_policy_core::authorizer::Decision;
use cedar_policy_core::parser::err::ParseErrors;

use rocket::serde::json::serde_json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthorizationCall {
    principal: Option<String>,
    action: Option<String>,
    resource: Option<String>,
    context: Option<serde_json::Value>,
    /// Optional schema in JSON format.
    /// If present, this will inform the parsing: for instance, it will allow
    /// `__entity` and `__extn` escapes to be implicit, and it will error if
    /// attributes have the wrong types (e.g., string instead of integer).
    /// currently unsupported
    #[schemars(skip)]
    policies: Option<String>,
    /// JSON object containing the entities data, in "natural JSON" form -- same
    /// format as expected by EntityJsonParser
    /// currently unsupported
    #[schemars(skip)]
    entities: Option<serde_json::Value>,
}

fn string_to_euid(optional_str: Option<String>) -> Result<Option<EntityUid>, ParseErrors> {
    match optional_str {
        Some(p) => match EntityUid::from_str(&p) {
            Ok(euid) => Ok(Some(euid)),
            Err(e) => Err(e),
        },
        None => Ok(None),
    }
}

impl TryInto<Request> for AuthorizationCall {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Request, Self::Error> {
        let principal = match string_to_euid(self.principal) {
            Ok(p) => p,
            Err(e) => return Err(e.into()),
        };
        let action = match string_to_euid(self.action) {
            Ok(a) => a,
            Err(e) => return Err(e.into()),
        };
        let resource = match string_to_euid(self.resource) {
            Ok(r) => r,
            Err(e) => return Err(e.into()),
        };
        let context = match self.context {
            Some(c) => match Context::from_json_value(c, None) {
                Ok(c) => c,
                Err(e) => return Err(e.into()),
            },
            None => Context::empty(),
        };
        Ok(Request::new(principal, action, resource, context))
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum DecisionRef {
    Allow,
    /// The `Authorizer` determined that the query should be denied.
    /// This is also returned if sufficiently fatal errors are encountered such
    /// that no decision could be safely reached; for example, errors parsing
    /// the policies.
    Deny,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DiagnosticsRef {
    /// `PolicyId`s of the policies that contributed to the decision.
    /// If no policies applied to the query, this set will be empty.
    reason: HashSet<String>,
    /// list of error messages which occurred
    errors: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AuthorizationAnswer {
    decision: DecisionRef,
    diagnostics: DiagnosticsRef,
}

impl Into<Response> for AuthorizationAnswer {
    fn into(self) -> Response {
        Response::new(
            match self.decision {
                DecisionRef::Allow => Decision::Allow,
                DecisionRef::Deny => Decision::Deny,
            },
            HashSet::from_iter(
                self.diagnostics
                    .reason
                    .iter()
                    .map(|r| cedar_policy::PolicyId::from_str(r).unwrap()),
            ),
            self.diagnostics.errors,
        )
    }
}

impl From<Response> for AuthorizationAnswer {
    fn from(value: Response) -> Self {
        AuthorizationAnswer {
            decision: match value.decision() {
                Decision::Allow => DecisionRef::Allow,
                Decision::Deny => DecisionRef::Deny,
            },
            diagnostics: DiagnosticsRef {
                reason: HashSet::from_iter(value.diagnostics().reason().map(|r| r.to_string())),
                errors: HashSet::from_iter(value.diagnostics().errors().map(|e| match e {
                    EvaluationError::StringMessage(e) => e,
                })),
            },
        }
    }
}
