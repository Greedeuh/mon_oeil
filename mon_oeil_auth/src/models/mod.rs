use serde::{Deserialize, Serialize};

mod mappers;
pub use mappers::*;

#[derive(Deserialize, Serialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Error {
    Bug(String),
    Auth,
}
