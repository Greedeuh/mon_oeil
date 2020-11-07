use actix_web::{error, web, web::Json, Error, HttpRequest, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use log::error;
use serde::Deserialize;

use crate::{ApiError, Conf};
use mon_oeil_core::{db, handlers, models::*};

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

impl Into<Error> for ApiError<mon_oeil_core::Error> {
    fn into(self) -> Error {
        match self.0 {
            mon_oeil_core::Error::Bug(e) => {
                error!("{:?}", e);
                error::ErrorInternalServerError("")
            }
            mon_oeil_core::Error::Auth => error::ErrorUnauthorized(""),
        }
    }
}

impl From<mon_oeil_core::Error> for ApiError<mon_oeil_core::Error> {
    fn from(err: mon_oeil_core::Error) -> ApiError<mon_oeil_core::Error> {
        ApiError(err)
    }
}

async fn get_gestures(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
) -> Result<impl Responder, ApiError<mon_oeil_core::Error>> {
    handlers::get_gestures(&db)
        .await
        .map(|res| Json(res))
        .map_err(ApiError::from)
}

async fn post_gesture(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    gesture: web::Json<NewGesture>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::post_gesture(
        &db,
        gesture.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|_| HttpResponse::Created().finish())
    .map_err(ApiError::from)
}

async fn delete_gesture(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id: web::Path<String>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::delete_gesture(&db, &id, &conf.hs256_private_key, credentials.token())
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(ApiError::from)
}

async fn post_description(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id_gesture: web::Path<String>,
    description: web::Json<NewDescription>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::post_description(
        &db,
        &id_gesture,
        description.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|_| HttpResponse::Created().finish())
    .map_err(ApiError::from)
}

async fn delete_description(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id: web::Path<String>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::delete_description(&db, &id, &conf.hs256_private_key, credentials.token())
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(ApiError::from)
}

async fn post_gesture_s_meaning(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id_gesture: web::Path<String>,
    meaning: web::Json<NewMeaning>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::post_gesture_s_meaning(
        &db,
        &id_gesture,
        meaning.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|_| HttpResponse::Created().finish())
    .map_err(ApiError::from)
}

async fn post_description_s_meaning(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id_descirption: web::Path<String>,
    meaning: web::Json<NewMeaning>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::post_description_s_meaning(
        &db,
        &id_descirption,
        meaning.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|_| HttpResponse::Created().finish())
    .map_err(ApiError::from)
}

async fn delete_meaning(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id: web::Path<String>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::delete_meaning(&db, &id, &conf.hs256_private_key, credentials.token())
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(ApiError::from)
}

async fn post_picture_meta(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id_gesture: web::Path<String>,
    picture: web::Json<NewPicture>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::post_picture_meta(
        &db,
        &id_gesture,
        picture.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|_| HttpResponse::Created().finish())
    .map_err(ApiError::from)
}

async fn delete_picture(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id: web::Path<String>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::delete_picture(&db, &id, &conf.hs256_private_key, credentials.token())
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(ApiError::from)
}
