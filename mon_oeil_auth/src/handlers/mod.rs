use actix_web::{error, Error};
use actix_web::{web, Result};
use chrono::prelude::*;
use log::error;

use crate::{db, models::*, Conf};
use mon_oeil_auth_shared::*;

pub async fn login(
    credential: web::Json<Credentials>,
    conf: web::Data<Conf>,
    db: web::Data<db::DbPool>,
) -> Result<String, ApiError> {
    let client = db.get().await.map_err(ApiError::from)?;

    let user = match client.get_user(&credential.username).await {
        Ok(Some(u)) => u,
        Ok(None) => return Err(ApiError::Auth),
        e => return Err(ApiError::Bug(format!("{:?}", e))),
    };

    if user.password != credential.password {
        return Err(ApiError::Auth);
    }

    let expire = Utc::now()
        .checked_add_signed(chrono::Duration::weeks(1000))
        .ok_or_else(|| ApiError::Auth)?;
    encode_jwt(
        &conf.hs256_private_key,
        JwtPayload {
            level: Level::Admin,
            exp: expire.timestamp(),
        },
    )
    .map_err(|e| ApiError::Bug(format!("{:?}", e)))
}

impl Into<Error> for ApiError {
    fn into(self) -> Error {
        match self {
            ApiError::Bug(e) => {
                error!("{:?}", e);
                error::ErrorInternalServerError("")
            }
            ApiError::Auth => error::ErrorUnauthorized(""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, web, App};

    #[actix_rt::test]
    async fn login_as_admin() {
        let mut pool = db::DbPool::faux();
        let mut client = db::DbClient::faux();
        unsafe {
            faux::when!(client.get_user).then(|_| {
                Ok(Some(User {
                    username: "jealpuducul".to_owned(),
                    password: "ahah".to_owned(),
                }))
            });
        }
        faux::when!(pool.get).once().safe_then(move |_| Ok(client));

        let mut app = test::init_service(
            App::new()
                .data(pool)
                .data(Conf {
                    hs256_private_key: "private_key".to_owned(),
                })
                .route("/", web::post().to(login)),
        )
        .await;
        let req = test::TestRequest::post()
            .header("content-type", "application/json")
            .set_json(&Credentials {
                username: "jealpuducul".to_owned(),
                password: "ahah".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let jwt = test::read_body(resp).await;

        let payload = decode_jwt("private_key", &std::str::from_utf8(&jwt).unwrap()).unwrap();
        assert_eq!(Level::Admin, payload.level);
    }

    #[actix_rt::test]
    async fn login_wrong_user() {
        let mut pool = db::DbPool::faux();
        let mut client = db::DbClient::faux();
        unsafe {
            faux::when!(client.get_user).then(|_| {
                Ok(Some(User {
                    username: "jealpuducul".to_owned(),
                    password: "ahah".to_owned(),
                }))
            });
        }
        faux::when!(pool.get).once().safe_then(move |_| Ok(client));

        let mut app = test::init_service(
            App::new()
                .data(pool)
                .data(Conf {
                    hs256_private_key: "private_key".to_owned(),
                })
                .route("/", web::post().to(login)),
        )
        .await;
        let req = test::TestRequest::post()
            .header("content-type", "application/json")
            .set_json(&Credentials {
                username: "jealpuducul".to_owned(),
                password: "BENNON".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_rt::test]
    async fn login_wrong_password() {
        let mut pool = db::DbPool::faux();
        let mut client = db::DbClient::faux();
        unsafe {
            faux::when!(client.get_user).then(|_| Ok(None));
        }
        faux::when!(pool.get).once().safe_then(move |_| Ok(client));

        let mut app = test::init_service(
            App::new()
                .data(pool)
                .data(Conf {
                    hs256_private_key: "private_key".to_owned(),
                })
                .route("/", web::post().to(login)),
        )
        .await;
        let req = test::TestRequest::post()
            .header("content-type", "application/json")
            .set_json(&Credentials {
                username: "jealpuducul".to_owned(),
                password: "ahah".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
