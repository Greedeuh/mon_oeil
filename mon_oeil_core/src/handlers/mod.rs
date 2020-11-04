use actix_web::{web, web::Json, HttpRequest, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{db, model::*, ApiError, Conf};
use mon_oeil_auth_shared::valid_jwt_admin;

pub async fn get_gestures(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
) -> Result<impl Responder, ApiError> {
    let gestures = db.get().await.map_err(ApiError::from)?;
    let gestures = gestures.gestures().await.map_err(ApiError::from)?;

    Ok(Json(gestures))
}

pub async fn post_gesture(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    gesture: web::Json<NewGesture>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials).map_err(ApiError::from)?;

    let gesture = gesture.into_inner();

    let client = db.get().await.map_err(ApiError::from)?;
    client.add_gesture(gesture).await.map_err(ApiError::from)?;

    Ok(HttpResponse::Created().finish())
}

pub async fn delete_gesture(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id: web::Path<String>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials).map_err(ApiError::from)?;

    let client = db.get().await.map_err(ApiError::from)?;
    client
        .delete_gesture_cascade(&id)
        .await
        .map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn post_description(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id_gesture: web::Path<String>,
    description: web::Json<NewDescription>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials).map_err(ApiError::from)?;

    let description = description.into_inner();

    let client = db.get().await.map_err(ApiError::from)?;
    client
        .add_description(description, &id_gesture)
        .await
        .map_err(ApiError::from)?;

    Ok(HttpResponse::Created().finish())
}

pub async fn delete_description(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id: web::Path<String>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials).map_err(ApiError::from)?;

    let client = db.get().await.map_err(ApiError::from)?;
    client
        .delete_description_cascade(&id)
        .await
        .map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn post_gesture_s_meaning(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id_gesture: web::Path<String>,
    meaning: web::Json<NewMeaning>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials).map_err(ApiError::from)?;

    let meaning = meaning.into_inner();

    let client = db.get().await.map_err(ApiError::from)?;
    client
        .add_meaning(meaning, Some(&id_gesture), None)
        .await
        .map_err(ApiError::from)?;

    Ok(HttpResponse::Created().finish())
}

pub async fn post_description_s_meaning(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id_descirption: web::Path<String>,
    meaning: web::Json<NewMeaning>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials).map_err(ApiError::from)?;

    let meaning = meaning.into_inner();

    let client = db.get().await.map_err(ApiError::from)?;
    client
        .add_meaning(meaning, None, Some(&id_descirption))
        .await
        .map_err(ApiError::from)?;

    Ok(HttpResponse::Created().finish())
}

pub async fn delete_meaning(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id: web::Path<String>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials).map_err(ApiError::from)?;

    let client = db.get().await.map_err(ApiError::from)?;
    client.delete_meaning(&id).await.map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn post_picture_meta(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id_gesture: web::Path<String>,
    picture: web::Json<NewPicture>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials).map_err(ApiError::from)?;

    let picture = picture.into_inner();

    let client = db.get().await.map_err(ApiError::from)?;
    client
        .add_picture(picture, &id_gesture)
        .await
        .map_err(ApiError::from)?;

    Ok(HttpResponse::Created().finish())
}

pub async fn delete_picture(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id: web::Path<String>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials).map_err(ApiError::from)?;
    let client = db.get().await.map_err(ApiError::from)?;
    client.delete_picture(&id).await.map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().finish())
}
