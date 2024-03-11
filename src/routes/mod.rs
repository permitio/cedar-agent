use rocket::get;
use rocket::response::status;
use rocket_okapi::openapi;

pub mod authorization;
pub mod data;
pub mod policies;
pub mod schema;

#[openapi]
#[get("/")]
pub async fn healthy() -> status::NoContent {
    status::NoContent
}
