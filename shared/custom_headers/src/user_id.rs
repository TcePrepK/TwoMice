use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpRequest};
use std::future::{ready, Ready};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserId(pub Uuid);

impl FromRequest for UserId {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, actix_web::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let header = match req.headers().get("X-User-Id") {
            Some(v) => v,
            None => return ready(Err(actix_web::error::ErrorBadRequest("missing X-User-Id"))),
        };

        let s = match header.to_str() {
            Ok(s) => s,
            Err(_) => return ready(Err(actix_web::error::ErrorBadRequest("invalid X-User-Id"))),
        };

        let uuid = match Uuid::parse_str(s) {
            Ok(u) => u,
            Err(_) => return ready(Err(actix_web::error::ErrorBadRequest("invalid X-User-Id"))),
        };

        ready(Ok(UserId(uuid)))
    }
}
