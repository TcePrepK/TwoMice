use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use std::future::{ready, Ready};

#[derive(Debug)]
pub struct SessionToken(pub String);

impl FromRequest for SessionToken {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, actix_web::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let header = match req.headers().get("X-Session-Token") {
            Some(v) => v,
            None => {
                return ready(Err(actix_web::error::ErrorBadRequest(
                    "missing X-Session-Token",
                )));
            }
        };

        let token = match header.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => {
                return ready(Err(actix_web::error::ErrorBadRequest(
                    "invalid X-Session-Token",
                )));
            }
        };

        ready(Ok(SessionToken(token)))
    }
}

impl From<SessionToken> for String {
    fn from(value: SessionToken) -> Self {
        value.0
    }
}
