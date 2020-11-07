use actix_cors::{Cors, CorsFactory};
use actix_web::http;

pub mod auth;
pub mod core;

pub fn cors() -> CorsFactory {
    Cors::new()
        .allowed_origin("*")
        .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
        .finish()
}

pub struct Conf {
    pub hs256_private_key: String,
}

struct ApiError<T>(T);
