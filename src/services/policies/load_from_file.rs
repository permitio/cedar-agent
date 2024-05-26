use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use log::{error, info};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::serde::json::Json;
use rocket::Rocket;
use rocket::Build;

use crate::services::policies::PolicyStore;
use crate::services::schema::SchemaStore;
use crate::schemas::policies::Policy;
use crate::config;

pub struct InitPoliciesFairing;

pub(crate) async fn init(
    conf: &config::Config,
    policy_store: &Box<dyn PolicyStore>,
    schema_store: &Box<dyn SchemaStore>
) {
    if conf.policies.is_none() {
        return;
    }

    let file_path = conf.policies.clone().unwrap();
    let policies_file_path = &file_path;
    let policies = match load_policies_from_file(policies_file_path.to_path_buf()).await {
        Ok(policies) => policies,
        Err(err) => {
            error!("Failed to load policies from file: {}", err);
            return;
        }
    };

    let schema = schema_store.get_cedar_schema().await;
    match policy_store.update_policies(policies.into_inner(), schema).await {
        Ok(policies) => {
            info!("Successfully updated policies from file {}: {} policies", &file_path.display(), policies.len());
        }
        Err(err) => {
            error!("Failed to update policies: {}", err);
            return;
        }
    };
}

pub async fn load_policies_from_file(path: PathBuf) -> Result<Json<Vec<Policy>>, Box<dyn Error>> {

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

    let policies: Vec<Policy> = match rocket::serde::json::from_str(&contents) {
        Ok(policies) => policies,
        Err(err) => return Err(format!("Failed to deserialize JSON: {}", err).into()),
    };
    
    Ok(Json(policies))
}

#[async_trait::async_trait]
impl Fairing for InitPoliciesFairing {
    fn info(&self) -> Info {
        Info {
            name: "Init Policies",
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
            rocket.state::<Box<dyn PolicyStore>>().unwrap(),
            rocket.state::<Box<dyn SchemaStore>>().unwrap()
        ).await;

        Ok(rocket)
    }
}
