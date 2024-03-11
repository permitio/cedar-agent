use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use log::{error, info};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use rocket::Build;

use crate::services::data::DataStore;
use crate::services::schema::SchemaStore;
use crate::config;
use crate::schemas::data::Entities;

pub struct InitDataFairing;

pub(crate) async fn init(
    conf: &config::Config,
    data_store: &Box<dyn DataStore>,
    schema_store: &Box<dyn SchemaStore>
) {

    if conf.data.is_none() {
        return;
    }

    let file_path = conf.data.clone().unwrap();
    let entities_file_path = &file_path;
    let entities = match load_entities_from_file(entities_file_path.to_path_buf()).await {
        Ok(entities) => entities,
        Err(err) => {
            error!("Failed to load entities from file: {}", err);
            return;
        }
    };

    let schema = schema_store.get_cedar_schema().await;
    match data_store.update_entities(entities, schema).await {
        Ok(entities) => {
            info!("Successfully updated entities from file {}: {} entities", &file_path.display(), entities.len());
        }
        Err(err) => {
            error!("Failed to update entities: {}", err);
            return;
        }
    };
}

pub async fn load_entities_from_file(path: PathBuf) -> Result<Entities, Box<dyn Error>> {
    
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
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(format!("Failed to read file: {}", err).into());
    }

    let entities: Entities = match rocket::serde::json::from_str(&contents) {
        Ok(entities) => entities,
        Err(err) => return Err(format!("Failed to deserialize JSON: {}", err).into()),
    };
    
    Ok(entities)
}

#[async_trait::async_trait]
impl Fairing for InitDataFairing {
    fn info(&self) -> Info {
        Info {
            name: "Init Data",
            kind: Kind::Ignite
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        let config = rocket.state::<config::Config>();

        if config.is_none() {
            return Ok(rocket);
        }

        init(
            config.unwrap(),
            rocket.state::<Box<dyn DataStore>>().unwrap(),
            rocket.state::<Box<dyn SchemaStore>>().unwrap()
        ).await;

        Ok(rocket)
    }
}
