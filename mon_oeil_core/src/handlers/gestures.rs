use crate::{models::*, Error};
use mon_oeil_auth_shared::valid_jwt_admin;
use mon_oeil_db as db;
use mon_oeil_storage::*;

pub async fn get_gestures(
    db: &db::GestureClientPool,
    storage: &Storage,
) -> Result<Vec<Gesture>, Error> {
    let gestures = db.get().await.map_err(Error::from)?;
    let gestures = gestures.gestures().await?;
    let gestures = gestures
        .into_iter()
        .map(|gesture_db| merge_db_and_storage(gesture_db, &storage))
        .collect();
    Ok(gestures)
}

pub fn merge_db_and_storage(gesture_db: db::Gesture, storage: &Storage) -> Gesture {
    let db::Gesture {
        id,
        tags,
        descriptions,
        meanings,
        pictures,
    } = gesture_db;
    Gesture {
        id,
        tags,
        descriptions: descriptions.into_iter().map(From::from).collect(),
        meanings: meanings.into_iter().map(From::from).collect(),
        pictures: pictures
            .into_iter()
            .map(|picture_db| {
                let storage_url = storage.get_url(&picture_db.id, &picture_db.format);
                Picture::from(picture_db, storage_url)
            })
            .collect(),
    }
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
