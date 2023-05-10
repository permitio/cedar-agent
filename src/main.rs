extern crate core;
extern crate rocket;

use std::borrow::Borrow;

use rocket::catchers;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi_get_routes, rapidoc::*, swagger_ui::*};

use crate::services::data::memory::MemoryDataStore;
use crate::services::data::DataStore;
use crate::services::policies::memory::MemoryPolicyStore;
use crate::services::policies::PolicyStore;

mod authn;
mod common;
mod config;
mod errors;
mod logger;
mod routes;
mod schemas;
mod services;

#[rocket::main]
async fn main() {
    let config = config::init();
    logger::init(&config);
    let server_config: rocket::figment::Figment = config.borrow().into();
    let launch_result = rocket::custom(server_config)
        .manage(config)
        .manage(Box::new(MemoryPolicyStore::new()) as Box<dyn PolicyStore>)
        .manage(Box::new(MemoryDataStore::new()) as Box<dyn DataStore>)
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
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}
