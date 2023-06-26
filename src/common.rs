use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use async_trait::async_trait;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header};
use rocket::{Data, Request};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) struct EmptyError;

impl Display for EmptyError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        return Ok(());
    }
}

pub(crate) struct DefaultContentType(ContentType);

impl DefaultContentType {
    /// Set a default content type for incoming messages
    pub fn new(t: ContentType) -> Self {
        Self(t)
    }
}

impl DefaultContentType {
    fn is_matching_wildcard(&self, accept_headers: Vec<&str>) -> bool {
        for accept_header in accept_headers {
            let header = ContentType::from_str(accept_header);
            match header {
                Ok(header) => {
                    if header.top() == "*" && header.sub() == "*" {
                        // Accepts all content types should be transformed
                        // to the default content type
                        return true;
                    }
                    if header.top() == self.0.top() && header.sub() == "*" {
                        // Accepts all subtypes of the default content type
                        return true;
                    }
                    if header.top() == "*" && header.sub() == self.0.sub() {
                        // Accepts all types of the default content type
                        return true;
                    }
                }
                Err(_) => {
                    // Convert invalid headers to the default content type
                    return true;
                }
            };
        }
        return false;
    }
}

#[async_trait]
impl Fairing for DefaultContentType {
    fn info(&self) -> Info {
        Info {
            name: "DefaultContentType",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        let accept_header = req.headers().get_one("Accept");
        if accept_header.is_none()
            || self.is_matching_wildcard(accept_header.unwrap().split(", ").collect())
        {
            req.replace_header(Header::new("Accept", format!("{}", self.0)));
        }
    }
}
