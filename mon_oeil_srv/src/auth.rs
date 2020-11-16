use actix_web::{error, web, web::Json, Error, HttpRequest, Responder};
use log::error;

use crate::{ApiError, Conf};
use mon_oeil_auth::*;
use mon_oeil_db as db;

pub fn app_config(config: &mut web::ServiceConfig, hs256_private_key: &str) {
    config
        .data(Conf {
            hs256_private_key: hs256_private_key.to_owned(),
        })
        .route("/login", web::post().to(login));
}

impl Into<Error> for ApiError<mon_oeil_auth::Error> {
    fn into(self) -> Error {
        match self.0 {
            mon_oeil_auth::Error::Bug(e) => {
                error!("{:?}", e);
                error::ErrorInternalServerError("")
            }
            mon_oeil_auth::Error::Auth => error::ErrorUnauthorized(""),
        }
    }
}

impl From<mon_oeil_auth::Error> for ApiError<mon_oeil_auth::Error> {
    fn from(err: mon_oeil_auth::Error) -> ApiError<mon_oeil_auth::Error> {
        ApiError(err)
    }
}

async fn login(
    _req: HttpRequest,
    credentials: Json<Credentials>,
    db: web::Data<db::GestureClientPool>,
    conf: web::Data<Conf>,
) -> Result<impl Responder, ApiError<mon_oeil_auth::Error>> {
    handlers::login(&credentials, &conf.hs256_private_key, &db)
        .await
        .map(Json)
        .map_err(ApiError::from)
}
