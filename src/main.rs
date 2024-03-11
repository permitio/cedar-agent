extern crate core;
extern crate rocket;

use std::borrow::Borrow;
use std::process::ExitCode;

use log::{error, info};
use rocket::catchers;
use rocket::http::ContentType;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi_get_routes, rapidoc::*, swagger_ui::*};

use crate::services::data::memory::MemoryDataStore;
use crate::services::data::DataStore;
use crate::services::policies::memory::MemoryPolicyStore;
use crate::services::policies::PolicyStore;
use crate::services::schema::memory::MemorySchemaStore;
use crate::services::schema::SchemaStore;

mod authn;
mod common;
mod config;
mod errors;
mod logger;
mod routes;
mod schemas;
mod services;

#[rocket::main]
async fn main() -> ExitCode {
    let config = config::init();
    logger::init(&config);
    let server_config: rocket::figment::Figment = config.borrow().into();
    let launch_result = rocket::custom(server_config)
        .attach(common::DefaultContentType::new(ContentType::JSON))
        .attach(services::schema::load_from_file::InitSchemaFairing)
        .attach(services::data::load_from_file::InitDataFairing)
        .attach(services::policies::load_from_file::InitPoliciesFairing)
        .manage(config)
        .manage(Box::new(MemoryPolicyStore::new()) as Box<dyn PolicyStore>)
        .manage(Box::new(MemoryDataStore::new()) as Box<dyn DataStore>)
        .manage(Box::new(MemorySchemaStore::new()) as Box<dyn SchemaStore>)
        .manage(cedar_policy::Authorizer::new())
        .register(
            "/",
            catchers![
                errors::catchers::handle_500,
                errors::catchers::handle_404,
                errors::catchers::handle_400,
            ],
        )
        .mount(
            "/v1",
            openapi_get_routes![
                routes::healthy,
                routes::policies::get_policies,
                routes::policies::get_policy,
                routes::policies::create_policy,
                routes::policies::update_policies,
                routes::policies::update_policy,
                routes::policies::delete_policy,
                routes::data::get_entities,
                routes::data::update_entities,
                routes::data::delete_entities,
                routes::authorization::is_authorized,
                routes::schema::get_schema,
                routes::schema::update_schema,
                routes::schema::delete_schema
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../v1/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .launch()
        .await;
    return match launch_result {
        Ok(_) => {
            info!("Cedar-Agent shut down gracefully.");
            ExitCode::SUCCESS
        }
        Err(err) => {
            error!("Cedar-Agent shut down with error: {}", err);
            ExitCode::FAILURE
        }
    };
}
