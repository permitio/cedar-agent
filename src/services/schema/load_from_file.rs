use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use async_trait::async_trait;
use log::{error, info};
use rocket::{Build, Rocket};
use rocket::fairing::{Fairing, Info, Kind};

use crate::config;
use crate::schemas::schema::Schema;
use crate::services::SchemaStore;

pub struct InitSchemaFairing;

pub(crate) async fn init(conf: &config::Config, schema_store: &Box<dyn SchemaStore>) {
    if conf.schema.is_none() {
        return;
    }

    let file_path = conf.schema.clone().unwrap();
    let schema_file_path = &file_path;
    let schema = match load_schema_from_file(schema_file_path.to_path_buf()).await {
        Ok(schema) => schema,
        Err(err) => {
            error!("Failed to load schema from file: {}", err);
            return;
        },
    };

    match schema_store.update_schema(schema).await {
        Ok(_) => {
            info!("Successfully updated schema from file {}", &file_path.display());
        },
        Err(err) => {
            error!("Failed to update schema: {}", err);
        },
    }
}

pub async fn load_schema_from_file(path: PathBuf) -> Result<Schema, Box<dyn Error>> {
    if !path.try_exists().unwrap_or(false) || !path.is_file() {
        return Err("File does not exist".into());
    }

    if path.extension().unwrap() != "json" {
        return Err("File is not a json file".into());
    }

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Failed to open file: {}", err).into()),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => match rocket::serde::json::from_str(&contents) {
                Ok(schema) => Ok(schema),
                Err(err) => Err(format!("Failed to deserialize JSON: {}", err).into()),
            }
        Err(err) => Err(format!("Failed to read file {}", err).into())
    }
}

#[async_trait]
impl Fairing for InitSchemaFairing {
    fn info(&self) -> Info {
        Info {
            name: "Init Schema",
            kind: Kind::Ignite
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        let config = rocket.state::<config::Config>();

        if config.is_some() {
            init(config.unwrap(), rocket.state::<Box<dyn SchemaStore>>().unwrap()).await;
        }

        Ok(rocket)
    }
}
