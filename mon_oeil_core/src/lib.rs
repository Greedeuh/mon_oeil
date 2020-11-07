use serde::{Deserialize, Serialize};

pub mod db;
pub mod handlers;
pub mod models;

use mon_oeil_auth_shared as auth;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Error {
    Bug(String),
    Auth,
}

impl From<models::DbError> for Error {
    fn from(err: models::DbError) -> Error {
        Error::Bug(format!("{:?}", err))
    }
}

impl From<auth::JwtValidationError> for Error {
    fn from(_err: auth::JwtValidationError) -> Error {
        Error::Auth
    }
}
