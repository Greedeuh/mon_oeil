use actix_multipart::Multipart;
use actix_web::{
    error, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Result,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use dotenv;
use failure::Fail;
use futures::{StreamExt, TryStreamExt};
use mon_oeil_auth_shared;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod storage;

use storage::Storage;

async fn upload(
    _req: HttpRequest,
    mut payload: Multipart,
    storage: web::Data<Storage>,
    credentials: BearerAuth,
    conf: web::Data<Conf>,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials)?;

    // iterate over multipart stream
    while let Ok(Some(field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| ApiError("Fail parsing content".to_owned()))?;

        let filename = content_type
            .get_filename()
            .ok_or_else(|| ApiError("Fail parsing filename".to_owned()))?;

        check_filename(&filename)?;

        storage
            .save_picture(filename, field.map_err(|_| ()).collect().await)
            .await
            .map_err(|_| ApiError("Fail parsing filename".to_owned()))?;
    }

    Ok(HttpResponse::Ok().finish())
}

async fn delete(
    _req: HttpRequest,
    id: web::Path<String>,
    storage: web::Data<Storage>,
    credentials: BearerAuth,
    conf: web::Data<Conf>,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials)?;
    storage
        .delete_picture(&id)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|err| ApiError(format!("{:?}", err)))
}

fn check_filename(filename: &str) -> Result<(), ApiError> {
    // should be as {uuid}.png
    let re = Regex::new(r"(.+)\.png").unwrap();
    let id = re.captures(&filename).map(|r| Uuid::parse_str(&r[1]));
    match id {
        Some(Ok(_)) => Ok(()),
        _ => Err(ApiError("Bad file name ".to_owned())),
    }
}

struct Conf {
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
        .route("/", web::post().to(upload))
.route("/{id}", web::delete().to(delete))
;
}


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Fail)]
#[fail(display = "{}", _0)]
struct ApiError(pub String);
// Use default implementation for `error_response()` method
impl error::ResponseError for ApiError {}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    const ADMIN_TOKEN: &str = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjIyMDM0MDcxOTgsImxldmVsIjoiQWRtaW4ifQ.RLE2du-ICZ0mlFl02YytZC02Xk0U5qyNRBxhi_-SvY8";

    #[actix_rt::test]
    async fn upload_2_pictures() {
        let mut storage = Storage::faux();
        unsafe {
            faux::when!(storage.save_picture).then(|_| Ok(()));
        }
        let mut app = test::init_service(
            App::new()
                .data(storage)
                .data(Conf {
                    hs256_private_key: "private_key".to_owned(),
                })
                .wrap(Logger::default())
                .route("/", web::get().to(super::upload)),
        )
        .await;

        let req = test::TestRequest::with_header("content-type", "multipart/form-data")
            .header("Authorization", ADMIN_TOKEN)
            .to_request();
        // TODO
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    // #[actix_rt::test]
    // async fn upload_fail() {
    //     let mut storage = Storage::faux();
    //     unsafe {
    //         faux::when!(storage.save_picture).then(|_| Err(storage::StorageErr("fail".to_owned())));
    //     }

    //     let mut app = test::init_service(
    //         App::new()
    //             .data(storage)
    //             .wrap(Logger::default())
    //             .route("/", web::get().to(super::upload)),
    //     )
    //     .await;

    //     let req =
    //         test::TestRequest::with_header("content-type", "multipart/form-data").to_request();
    //     let resp = test::call_service(&mut app, req).await;

    //     assert!(resp.status().is_server_error());
    // }

    #[actix_rt::test]
    async fn delete_picture() {
        let mut storage = Storage::faux();
        unsafe {
            faux::when!(storage.delete_picture).then(|_| Ok(()));
        }

        let mut app = test::init_service(
            App::new()
                .data(storage)
                .data(Conf {
                    hs256_private_key: "private_key".to_owned(),
                })
                .wrap(Logger::default())
                .route("/{id}", web::get().to(super::delete)),
        )
        .await;

        let req = test::TestRequest::with_uri("/7cc8f901-07d3-4bba-85d5-9cf66f9470e0")
            .header("Authorization", ADMIN_TOKEN)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn delete_fail() {
        let mut storage = Storage::faux();
        unsafe {
            faux::when!(storage.delete_picture)
                .then(|_| Err(storage::StorageErr("fail".to_owned())));
        }

        let mut app = test::init_service(
            App::new()
                .data(storage)
                .data(Conf {
                    hs256_private_key: "private_key".to_owned(),
                })
                .wrap(Logger::default())
                .route("/{id}", web::get().to(super::delete)),
        )
        .await;

        let req = test::TestRequest::with_uri("/7cc8f901-07d3-4bba-85d5-9cf66f9470e0")
            .header("Authorization", ADMIN_TOKEN)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_server_error());
    }
}
