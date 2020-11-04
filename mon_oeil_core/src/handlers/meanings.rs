use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{db, models::*, ApiError, Conf};
use mon_oeil_auth_shared::valid_jwt_admin;

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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{middleware::Logger, test, web, App};

    const ADMIN_TOKEN: &str = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjIyMDM0MDcxOTgsImxldmVsIjoiQWRtaW4ifQ.RLE2du-ICZ0mlFl02YytZC02Xk0U5qyNRBxhi_-SvY8";

    #[actix_rt::test]
    async fn add_gesture_s_meaning() {
        let mut pool = db::DbPool::faux();
        let mut client = db::DbClient::faux();
        unsafe {
            faux::when!(client.add_meaning).then(|_| Ok(()));
        }
        faux::when!(pool.get).once().safe_then(move |_| Ok(client));

        let mut app = test::init_service(
            App::new()
                .data(pool)
                .data(Conf {
                    hs256_private_key: "private_key".to_owned(),
                })
                .wrap(Logger::default())
                .route("/{id_gesture}", web::post().to(post_gesture_s_meaning)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/une_id_gesture")
            .header("content-type", "application/json")
            .header("Authorization", ADMIN_TOKEN)
            .set_json(&NewMeaning {
                value: "Un petit meaning".to_owned(),
                langs: vec!["fr".to_owned(), "us".to_owned()],
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn add_decription_s_meaning() {
        let mut pool = db::DbPool::faux();
        let mut client = db::DbClient::faux();
        unsafe {
            faux::when!(client.add_meaning).then(|_| Ok(()));
        }
        faux::when!(pool.get).once().safe_then(move |_| Ok(client));

        let mut app = test::init_service(
            App::new()
                .data(pool)
                .data(Conf {
                    hs256_private_key: "private_key".to_owned(),
                })
                .wrap(Logger::default())
                .route("/{id_gesture}", web::post().to(post_description_s_meaning)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/une_id_gesture")
            .header("content-type", "application/json")
            .header("Authorization", ADMIN_TOKEN)
            .set_json(&NewMeaning {
                value: "Un petit meaning".to_owned(),
                langs: vec!["fr".to_owned(), "us".to_owned()],
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn delete_meaning() {
        let mut pool = db::DbPool::faux();
        let mut client = db::DbClient::faux();
        unsafe {
            // TODO : Comprendre quel est la différene avec le test post du dessus
            faux::when!(client.delete_meaning).then(|_| Ok(()));
        }
        faux::when!(pool.get).once().safe_then(move |_| Ok(client));

        let mut app = test::init_service(
            App::new()
                .data(pool)
                .data(Conf {
                    hs256_private_key: "private_key".to_owned(),
                })
                .wrap(Logger::default())
                .route("/{id}", web::delete().to(super::delete_meaning)),
        )
        .await;

        let req = test::TestRequest::delete()
            .uri("/une_id")
            .header("content-type", "application/json")
            .header("Authorization", ADMIN_TOKEN)
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        dbg!(resp.status());
        assert!(resp.status().is_success());
    }
}
