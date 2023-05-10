use rocket::catch;
use rocket::http::Status;

use rocket::Request;

use crate::errors::response::ErrorResponse;

#[catch(500)]
pub fn handle_500(status: Status, req: &Request<'_>) -> ErrorResponse {
    let req_url = req.uri();
    return ErrorResponse {
        reason: format!("An error occurred during handling {req_url}"),
        description: "An unexpected error has occurred".to_owned(),
        code: status.code,
    };
}

#[catch(400)]
pub fn handle_400(_req: &Request<'_>) -> ErrorResponse {
    return ErrorResponse {
        description: "The request could not be understood by the server due to malformed syntax."
            .to_owned(),
        reason: "The request content is not valid".to_owned(),
        code: 400,
    };
}

#[catch(404)]
pub fn handle_404(req: &Request<'_>) -> ErrorResponse {
    let req_url = req.uri();
    return ErrorResponse {
        description: format!("The requested resource {req_url} was not found"),
        reason: "The requested resource was not found".to_owned(),
        code: 404,
    };
}
