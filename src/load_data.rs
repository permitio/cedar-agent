use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use log::{error, info};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::Rocket;
use rocket::Build;

use crate::services::data::DataStore;
use crate::config;
use crate::schemas::data::Entities;

pub struct InitDataFairing;

pub(crate) async fn init(conf: &config::Config, data_store: &Box<dyn DataStore>) {
    let file_path = conf.data.clone().unwrap_or("".to_string());

    if file_path.is_empty() {
        return;
    }

    let entities_file_path = PathBuf::from(&file_path);
    let entities = match load_entities_from_file(entities_file_path).await {
        Ok(entities) => entities,
        Err(err) => {
            error!("Failed to load entities from file: {}", err);
            return;
        }
    };

    match data_store.update_entities(entities).await {
        Ok(entities) => {
            info!("Successfully updated entities from file {}: {} entities", &file_path, entities.len());
        }
        Err(err) => {
            error!("Failed to update entities: {}", err);
            return;
        }
    };
}

async fn load_entities_from_file(path: PathBuf) -> Result<Entities, Box<dyn Error>> {
    // check if file exists
    if !path.exists() {
        return Err("File does not exist".into());
    }

    // check if is a valid json file
    if path.extension().unwrap() != "json" {
        return Err("File is not a json file".into());
    }

    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let entities: Entities = serde_json::from_str(&contents)?;
    Ok(entities)
}

#[async_trait::async_trait]
impl Fairing for InitDataFairing {
    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        let config = rocket.state::<config::Config>().unwrap();
        init(config, rocket.state::<Box<dyn DataStore>>().unwrap()).await;

        Ok(rocket)
    }

    fn info(&self) -> Info {
        Info {
            name: "Init Data",
            kind: Kind::Ignite
        }
    }
}