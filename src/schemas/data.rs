use std::error::Error;

use cedar_policy_core::entities::{
    EntitiesError, EntityJSON, EntityJsonParser, NoEntitiesSchema, TCComputation,
};
use cedar_policy_core::extensions::Extensions;
use cedar_policy_core::{ast, entities};
use cedar_policy::Schema;
use log::debug;
use rocket::serde::json::serde_json::{from_str, json, to_string};
use rocket::serde::json::Value;

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::EmptyError;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Entity(Value);

impl From<ast::Entity> for Entity {
    fn from(value: ast::Entity) -> Self {
        let entity_json = EntityJSON::from_entity(&value).unwrap();
        let json_string = to_string(&entity_json).unwrap();
        Self(from_str(&json_string).unwrap())
    }
}

impl TryInto<ast::Entity> for Entity {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<ast::Entity, Self::Error> {
        debug!("Parsing entity into ast format");
        let parser: EntityJsonParser<NoEntitiesSchema> =
            EntityJsonParser::new(None, Extensions::all_available(), TCComputation::ComputeNow);
        let entities = match parser.from_json_value(self.0) {
            Ok(entities) => entities,
            Err(err) => return Err(err.into()),
        };
        for entity in entities.iter() {
            return Ok(entity.clone());
        }
        Err(EmptyError.into())
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Entities(Vec<Entity>);

impl Entities {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    // Custom conversion function in place of a TryInto implementation
    // This is due to the extra optional argument (schema)
    pub fn convert_to_cedar_entities(&self, schema: &Option<Schema>) -> Result<cedar_policy::Entities, EntitiesError> {
        debug!("Parsing entities into cedar format");
        cedar_policy::Entities::from_json_value(json!(self.0), schema.as_ref())
    }
}

impl From<entities::Entities> for Entities {
    fn from(value: entities::Entities) -> Self {
        Self(value.iter().map(|v| Entity::from(v.clone())).collect())
    }
}

impl TryInto<entities::Entities> for Entities {
    type Error = EntitiesError;

    fn try_into(self) -> Result<entities::Entities, Self::Error> {
        debug!("Parsing entities into ast format");
        let parser: EntityJsonParser<NoEntitiesSchema> =
            EntityJsonParser::new(None, Extensions::all_available(), TCComputation::ComputeNow);
        parser.from_json_value(json!(self.0))
    }
}
