use actix_web::{error, http, web};
use failure::Fail;
use serde::{Deserialize, Serialize};

mod db;
mod handlers;
mod models;

use handlers::*;
use mon_oeil_auth_shared as auth;

pub fn app_config(
    config: &mut web::ServiceConfig,
    db_pool: &mon_oeil_db::GestureClientPool,
    hs256_private_key: &str,
) {
    config
        .data(db::DbPool::new(db_pool.clone()))
        .data(Conf {
            hs256_private_key: hs256_private_key.to_owned(),
        })
        .route("/gestures", web::get().to(get_gestures))
        .route("/gestures", web::post().to(post_gesture))
        .route("/gestures/{id}", web::delete().to(delete_gesture))
        .route(
            "/gestures/{id_gesutre}/descriptions",
            web::post().to(post_description),
        )
        .route("/description", web::delete().to(delete_description))
        .route(
            "/gestures/{id_gesutre}/meanings",
            web::post().to(post_gesture_s_meaning),
        )
        .route(
            "/descriptions/{id_description}/meanings",
            web::post().to(post_description_s_meaning),
        )
        .route("/meanings", web::delete().to(delete_meaning))
        .route(
            "/gestures/{id_gesutre}/pictures",
            web::post().to(post_picture_meta),
        )
        .route("/pictures", web::delete().to(delete_picture));
}

pub struct Conf {
    pub hs256_private_key: String,
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

impl From<auth::JwtValidationError> for ApiError {
    fn from(err: auth::JwtValidationError) -> ApiError {
        ApiError(format!("{:?}", err))
    }
}
