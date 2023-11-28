use cedar_policy;
use log::debug;
use serde::{Deserialize, Serialize};

use rocket::serde::json::Value;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Schema(Value);

impl Schema {
    pub fn empty() -> Self {
        Self {
            0: Value::Null
        }
    }
}

impl TryInto<cedar_policy::Schema> for Schema {
    type Error = cedar_policy::SchemaError;

    fn try_into(self) -> Result<cedar_policy::Schema, Self::Error> {
        debug!("Parsing schema");
        cedar_policy::Schema::from_json_value(self.0)
    }
}