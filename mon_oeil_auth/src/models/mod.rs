use serde::{Deserialize, Serialize};
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
