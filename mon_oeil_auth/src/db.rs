use super::*;
use log::info;
use mon_oeil_db as db;
use std::env;

#[cfg_attr(test, faux::create)]
pub struct DbPool(db::GestureClientPool);

#[cfg_attr(test, faux::methods)]
impl DbPool {
    pub fn new(pool: db::GestureClientPool) -> Self {
        Self(pool)
    }

    pub async fn get(&self) -> Result<DbClient, DbError> {
        self.0
            .get()
            .await
            .map_err(DbError::from)
            .map(|client| DbClient::new(client))
    }
}

pub fn connect_db() -> db::GestureClientPool {
    dotenv::dotenv().ok();

    let (host, port, user, password, dbname) = (
        env::var("PG_HOST").unwrap(),
        env::var("PG_PORT").unwrap(),
        env::var("PG_DB_NAME").unwrap(),
        env::var("PG_USER").unwrap(),
        env::var("PG_PWD").unwrap(),
    );

    db::GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap()
}

#[cfg_attr(test, faux::create)]
pub struct DbClient(db::GestureClient);

#[cfg_attr(test, faux::methods)]
impl DbClient {
    pub fn new(client: db::GestureClient) -> Self {
        Self(client)
    }

    /// Delete picture from db
    pub async fn get_user(&self, username: &str) -> Result<Option<User>, DbError> {
        let user = self.0.get_user(username).await.map_err(DbError::from)?;
        Ok(user.map(User::from))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct DbError(String);

impl From<db::DbError> for DbError {
    fn from(err: db::DbError) -> DbError {
        DbError(format!("{:?}", err))
    }
}

impl From<db::User> for User {
    fn from(user: db::User) -> User {
        let db::User { username, password } = user;
        User { username, password }
    }
}
