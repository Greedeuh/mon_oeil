use actix_web::{error, web, Result};
use failure::Fail;
use serde::{Deserialize, Serialize};

mod db;
mod handlers;
mod models;

use handlers::*;

pub struct Conf {
    pub hs256_private_key: String,
}

pub fn app_config(
    config: &mut web::ServiceConfig,
    db_pool: &mon_oeil_db::GestureClientPool,
    hs256_private_key: &str,
) {
    config
        .data(Conf {
            hs256_private_key: hs256_private_key.to_owned(),
        })
        .data(db::DbPool::new(db_pool.clone()))
        .route("/login", web::post().to(login));
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Fail)]
#[fail(display = "my error")]
pub struct ApiError(pub String);

// Use default implementation for `error_response()` method
impl error::ResponseError for ApiError {}

impl From<db::DbError> for ApiError {
    fn from(err: db::DbError) -> ApiError {
        ApiError(format!("{:?}", err))
    }
}
