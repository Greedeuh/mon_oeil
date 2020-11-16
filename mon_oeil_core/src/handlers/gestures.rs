use crate::{models::*, Error};
use mon_oeil_auth_shared::valid_jwt_admin;
use mon_oeil_db as db;

pub async fn get_gestures(db: &db::GestureClientPool) -> Result<Vec<Gesture>, Error> {
    let gestures = db.get().await.map_err(Error::from)?;
    gestures
        .gestures()
        .await
        .map_err(Error::from)
        .map(|gestures| gestures.into_iter().map(Gesture::from).collect())
}

pub async fn post_gesture(
    db: &db::GestureClientPool,
    new_gesture: NewGesture,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<String, Error> {
    valid_jwt_admin(&hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .add_gesture(new_gesture.into())
        .await
        .map_err(Error::from)
}

pub async fn put_gesture(
    db: &db::GestureClientPool,
    id: &str,
    new_gesture: NewGesture,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(&hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .update_gesture(id, new_gesture.into())
        .await
        .map_err(Error::from)
}

pub async fn delete_gesture(
    db: &db::GestureClientPool,
    id: &str,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .delete_gesture_cascade(&id)
        .await
        .map_err(Error::from)
}
