use crate::{models::*, Error};
use mon_oeil_auth_shared::valid_jwt_admin;
use mon_oeil_db as db;

pub async fn post_gesture_s_meaning(
    db: &db::GestureClientPool,
    id_gesture: &str,
    new_meaning: NewMeaning,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<String, Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .add_meaning(new_meaning.into(), Some(&id_gesture), None)
        .await
        .map_err(Error::from)
}

pub async fn post_description_s_meaning(
    db: &db::GestureClientPool,
    id_descirption: &str,
    new_meaning: NewMeaning,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<String, Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .add_meaning(new_meaning.into(), None, Some(&id_descirption))
        .await
        .map_err(Error::from)
}

pub async fn put_meaning(
    db: &db::GestureClientPool,
    id: &str,
    new_meaning: NewMeaning,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(&hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .update_meaning(id, new_meaning.into())
        .await
        .map_err(Error::from)
}

pub async fn delete_meaning(
    db: &db::GestureClientPool,
    id: &str,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client.delete_meaning(&id).await.map_err(Error::from)
}
