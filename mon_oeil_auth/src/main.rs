use actix_cors::Cors;
use actix_web::{error, HttpResponse
    ,http, middleware::Logger, web, App, HttpServer, Result};
use chrono::prelude::*;
use dotenv;
use env_logger::Env;
use failure::Fail;
use mon_oeil_auth_shared::*;
use serde::{Deserialize, Serialize};
use std::env;


mod db;

async fn login(
    credential: web::Json<Credentials>,
    conf: web::Data<Conf>,
    db: web::Data<db::DbPool>,
) -> Result<String, ApiError> {
    let client = db.get().await.map_err(ApiError::from)?;

    let user = match client.get_user(&credential.username).await {
        Ok(Some(u)) => u,
        _ => return Err(ApiError("Fail to login ".to_owned())),
    };

    if user.password != credential.password {
        return Err(ApiError("Fail to login ".to_owned()));
    }

    let expire = Utc::now()
        .checked_add_signed(chrono::Duration::weeks(1000))
        .ok_or(ApiError("Expiration bug".to_owned()))?;
    encode_jwt(
        &conf.hs256_private_key,
        JwtPayload {
            level: Level::Admin,
            exp: expire.timestamp(),
        },
    )
    .map_err(|_| ApiError(format!("Failed to produce jwt")))
}

#[derive(Deserialize, Serialize)]
struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

struct Conf {
    pub hs256_private_key: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let hs256_private_key = env::var("HS256_PRIVATE_KEY").unwrap();
    let db_pool = db::connect_db();

    HttpServer::new(move || {
        App::new()
            .data(Conf {
                hs256_private_key: hs256_private_key.clone(),
            })
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .data(db::DbPool::new(db_pool.clone()))
            .wrap(Logger::default())
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

fn log() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Fail)]
#[fail(display = "my error")]
struct ApiError(pub String);

// Use default implementation for `error_response()` method
impl error::ResponseError for ApiError {}

impl From<db::DbError> for ApiError {
    fn from(err: db::DbError) -> ApiError {
        ApiError(format!("{:?}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

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
}
