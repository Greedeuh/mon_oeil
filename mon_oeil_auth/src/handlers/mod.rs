use actix_web::{web, Result};
use chrono::prelude::*;

use crate::{db, models::*, ApiError, Conf};
use mon_oeil_auth_shared::*;

pub async fn login(
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
        .ok_or_else(|| ApiError("Expiration bug".to_owned()))?;
    encode_jwt(
        &conf.hs256_private_key,
        JwtPayload {
            level: Level::Admin,
            exp: expire.timestamp(),
        },
    )
    .map_err(|_| ApiError("Failed to produce jwt".to_string()))
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
