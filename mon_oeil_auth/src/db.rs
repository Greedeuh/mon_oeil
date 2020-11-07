use crate::models::*;
use mon_oeil_db as db;

// Wrapper for mon_oeil_db::GestureClientPool & mon_oeil_db::GestureClient

#[cfg_attr(test, faux::create)]
pub struct DbPool(db::GestureClientPool);

#[cfg_attr(test, faux::methods)]
impl DbPool {
    pub fn new(pool: db::GestureClientPool) -> Self {
        Self(pool)
    }

    pub async fn get(&self) -> Result<DbClient, DbError> {
        self.0.get().await.map_err(DbError::from).map(DbClient::new)
    }
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
