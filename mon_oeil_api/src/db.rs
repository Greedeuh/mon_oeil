use crate::model::*;
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

    /// Retrieve all gestures from db
    pub async fn gestures(&self) -> Result<Vec<Gesture>, DbError> {
        info!("a");
        let gestures = self.0.gestures().await.map_err(DbError::from)?;
        info!("a");
        Ok(gestures.into_iter().map(From::from).collect())
    }

    /// Add a gesture and nested data in db
    pub async fn add_gesture(&self, gesture: NewGesture) -> Result<(), DbError> {
        self.0
            .add_gesture(gesture.into())
            .await
            .map_err(DbError::from)
    }

    /// Add a description and nested data in db for a gesture
    pub async fn add_description(
        &self,
        description: NewDescription,
        id_gesture: &str,
    ) -> Result<(), DbError> {
        self.0
            .add_description(description.into(), id_gesture)
            .await
            .map_err(DbError::from)
    }

    /// Add a meaning in db for a gesture or description
    pub async fn add_meaning(
        &self,
        meaning: NewMeaning,
        id_gesture: Option<&str>,
        id_description: Option<&str>,
    ) -> Result<(), DbError> {
        self.0
            .add_meaning(meaning.into(), id_gesture, id_description)
            .await
            .map_err(DbError::from)
    }

    /// Add a picture and nested data in db for a gesture
    pub async fn add_picture(&self, picture: NewPicture, id_gesture: &str) -> Result<(), DbError> {
        self.0
            .add_picture(picture.into(), id_gesture)
            .await
            .map_err(DbError::from)
    }

    /// Delete gesture and nested object from db
    pub async fn delete_gesture_cascade(&self, id: &str) -> Result<(), DbError> {
        self.0
            .delete_gesture_cascade(id)
            .await
            .map_err(DbError::from)
    }

    /// Delete description and nested data from db
    pub async fn delete_description_cascade(&self, id: &str) -> Result<(), DbError> {
        self.0
            .delete_description_cascade(id)
            .await
            .map_err(DbError::from)
    }

    /// Delete meaning from db
    pub async fn delete_meaning(&self, id: &str) -> Result<(), DbError> {
        self.0.delete_meaning(id).await.map_err(DbError::from)
    }

    /// Delete picture from db
    pub async fn delete_picture(&self, id: &str) -> Result<(), DbError> {
        self.0.delete_meaning(id).await.map_err(DbError::from)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct DbError(String);

impl From<db::DbError> for DbError {
    fn from(err: db::DbError) -> DbError {
        DbError(format!("{:?}", err))
    }
}
