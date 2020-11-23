use actix_multipart::Multipart;
use actix_web::{error, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use bytes::buf::BufMut;
use bytes::BytesMut;
use futures::{StreamExt, TryStreamExt};
use log::error;
use serde::Deserialize;

use crate::{ApiError, Conf};
use mon_oeil_core::*;
use mon_oeil_db as db;

pub fn app_config(config: &mut web::ServiceConfig) {
    config
        .route("/gestures", web::get().to(get_gestures))
        .route("/gestures", web::post().to(post_gesture))
        .route("/gestures/{id}", web::put().to(put_gesture))
        .route("/gestures/{id}", web::delete().to(delete_gesture))
        .route(
            "/gestures/{id_gesutre}/descriptions",
            web::post().to(post_description),
        )
        .route("/descriptions/{id}", web::put().to(put_description))
        .route("/descriptions/{id}", web::delete().to(delete_description))
        .route(
            "/gestures/{id_gesutre}/meanings",
            web::post().to(post_gesture_s_meaning),
        )
        .route(
            "/descriptions/{id_description}/meanings",
            web::post().to(post_description_s_meaning),
        )
        .route("/meanings/{id}", web::put().to(put_meaning))
        .route("/meanings/{id}", web::delete().to(delete_meaning))
        .route(
            "/gestures/{id_gesutre}/pictures",
            web::post().to(post_picture),
        )
        .route("/pictures/{id}/meta", web::put().to(put_picture_meta))
        .route("/pictures/{id}/file", web::put().to(put_picture_file))
        .route("/pictures/{id}", web::delete().to(delete_picture));
}

impl Into<Error> for ApiError<mon_oeil_core::Error> {
    fn into(self) -> Error {
        match self.0 {
            mon_oeil_core::Error::Bug(e) => {
                error!("{:?}", e);
                error::ErrorInternalServerError("")
            }
            mon_oeil_core::Error::Auth => error::ErrorUnauthorized(""),
            mon_oeil_core::Error::NotFound => error::ErrorNotFound(""),
            mon_oeil_core::Error::NotAccepted(x) => error::ErrorBadRequest(x),
        }
    }
}

impl From<mon_oeil_core::Error> for ApiError<mon_oeil_core::Error> {
    fn from(err: mon_oeil_core::Error) -> ApiError<mon_oeil_core::Error> {
        ApiError(err)
    }
}

async fn get_gestures(
    db: web::Data<db::GestureClientPool>,
    storage: web::Data<mon_oeil_storage::Storage>,
    search_param: web::Query<mon_oeil_core::SearchParam>,
) -> Result<impl Responder, ApiError<mon_oeil_core::Error>> {
    let max = search_param.max;
    handlers::get_gestures(&db, &storage, search_param.into_inner())
        .await
        .map(|(gestures, total)| {
            if max >= total {
                HttpResponse::Ok().json(gestures)
            } else {
                HttpResponse::PartialContent()
                    .header("Access-Control-Expose-Headers", "total-items")
                    .header("total-items", format!("{}", total))
                    .json(gestures)
            }
        })
        .map_err(ApiError::from)
}

async fn post_gesture(
    _req: HttpRequest,
    db: web::Data<db::GestureClientPool>,
    new_gesture: web::Json<NewGesture>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::post_gesture(
        &db,
        new_gesture.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|id| HttpResponse::Created().body(id))
    .map_err(ApiError::from)
}

async fn put_gesture(
    _req: HttpRequest,
    db: web::Data<db::GestureClientPool>,
    id: web::Path<String>,
    new_gesture: web::Json<NewGesture>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::put_gesture(
        &db,
        &id,
        new_gesture.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|_| HttpResponse::Created().finish())
    .map_err(ApiError::from)
}

async fn delete_gesture(
    _req: HttpRequest,
    db: web::Data<db::GestureClientPool>,
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
    db: web::Data<db::GestureClientPool>,
    id_gesture: web::Path<String>,
    new_description: web::Json<NewDescription>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::post_description(
        &db,
        &id_gesture,
        new_description.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|id| HttpResponse::Created().body(id))
    .map_err(ApiError::from)
}

async fn put_description(
    db: web::Data<db::GestureClientPool>,
    id: web::Path<String>,
    new_description: web::Json<NewDescription>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::put_description(
        &db,
        &id,
        new_description.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|_| HttpResponse::Created().finish())
    .map_err(ApiError::from)
}

async fn delete_description(
    _req: HttpRequest,
    db: web::Data<db::GestureClientPool>,
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
    db: web::Data<db::GestureClientPool>,
    id_gesture: web::Path<String>,
    new_meaning: web::Json<NewMeaning>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::post_gesture_s_meaning(
        &db,
        &id_gesture,
        new_meaning.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|id| HttpResponse::Created().body(id))
    .map_err(ApiError::from)
}

async fn post_description_s_meaning(
    _req: HttpRequest,
    db: web::Data<db::GestureClientPool>,
    id_descirption: web::Path<String>,
    new_meaning: web::Json<NewMeaning>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::post_description_s_meaning(
        &db,
        &id_descirption,
        new_meaning.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|id| HttpResponse::Created().body(id))
    .map_err(ApiError::from)
}

async fn put_meaning(
    db: web::Data<db::GestureClientPool>,
    id: web::Path<String>,
    new_meaning: web::Json<NewMeaning>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::put_meaning(
        &db,
        &id,
        new_meaning.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|_| HttpResponse::Created().finish())
    .map_err(ApiError::from)
}

async fn delete_meaning(
    _req: HttpRequest,
    db: web::Data<db::GestureClientPool>,
    id: web::Path<String>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::delete_meaning(&db, &id, &conf.hs256_private_key, credentials.token())
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(ApiError::from)
}

#[derive(Debug, Deserialize)]
struct NewPictureQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    langs: Option<String>,
}

async fn post_picture(
    files: Multipart,
    db: web::Data<db::GestureClientPool>,
    storage: web::Data<mon_oeil_storage::Storage>,
    id_gesture: web::Path<String>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
    new_picture: web::Query<NewPictureQuery>,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    match extract_file_and_format(files).await {
        Ok((content, format)) => {
            let new_picture = new_picture.into_inner();

            let new_picture = NewPicture {
                langs: new_picture
                    .langs
                    .unwrap_or_else(|| "".to_owned())
                    .split(';')
                    .map(str::to_owned)
                    .filter(|x| !x.is_empty())
                    .collect(),
                format,
            };
            handlers::post_picture(
                &db,
                &storage,
                &id_gesture,
                new_picture,
                content,
                &conf.hs256_private_key,
                credentials.token(),
            )
            .await
            .map(|id| HttpResponse::Created().body(id))
            .map_err(ApiError::from)
        }
        Err(res) => Ok(res),
    }
}

async fn put_picture_file(
    files: Multipart,
    id: web::Path<String>,
    db: web::Data<db::GestureClientPool>,
    storage: web::Data<mon_oeil_storage::Storage>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    match extract_file_and_format(files).await {
        Ok((content, format)) => handlers::put_picture_file(
            &db,
            &storage,
            &id,
            NewPictureFileInfo { format },
            content,
            &conf.hs256_private_key,
            credentials.token(),
        )
        .await
        .map(|_| HttpResponse::Created().finish())
        .map_err(ApiError::from),
        Err(res) => Ok(res),
    }
}

async fn extract_file_and_format(mut files: Multipart) -> Result<(Vec<u8>, String), HttpResponse> {
    if let Ok(Some(mut field)) = files.try_next().await {
        let mut content = BytesMut::new();
        while let Some(chunk) = field.next().await {
            match chunk {
                Ok(chunk) => content.put(chunk),
                _ => return Err(HttpResponse::BadRequest().body("File corrupted")),
            }
        }
        let content = content.freeze().to_vec();

        let content_type = field.content_type();
        let format = content_type.subtype().to_string();

        Ok((content, format))
    } else {
        Err(HttpResponse::BadRequest().body("File not found"))
    }
}

async fn put_picture_meta(
    db: web::Data<db::GestureClientPool>,
    id: web::Path<String>,
    new_picture_meta: web::Json<NewPictureMeta>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::put_picture_meta(
        &db,
        &id,
        new_picture_meta.into_inner(),
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|_| HttpResponse::Created().finish())
    .map_err(ApiError::from)
}

async fn delete_picture(
    _req: HttpRequest,
    db: web::Data<db::GestureClientPool>,
    storage: web::Data<mon_oeil_storage::Storage>,
    id: web::Path<String>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError<mon_oeil_core::Error>> {
    handlers::delete_picture(
        &db,
        &storage,
        &id,
        &conf.hs256_private_key,
        credentials.token(),
    )
    .await
    .map(|_| HttpResponse::Created().finish())
    .map_err(ApiError::from)
}
