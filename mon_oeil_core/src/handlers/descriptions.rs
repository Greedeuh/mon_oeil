use crate::{models::*, Error};
use mon_oeil_auth_shared::valid_jwt_admin;
use mon_oeil_db as db;

/// add description as auth user
pub async fn post_description(
    db: &db::GestureClientPool,
    id_gesture: &str,
    new_description: NewDescription,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<String, Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .add_description(new_description.into(), &id_gesture)
        .await
        .map_err(Error::from)
}

pub async fn put_description(
    db: &db::GestureClientPool,
    id: &str,
    new_description: NewDescription,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(&hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .update_description(id, new_description.into())
        .await
        .map_err(Error::from)
}

pub async fn delete_description(
    db: &db::GestureClientPool,
    id: &str,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .delete_description_cascade(&id)
        .await
        .map_err(Error::from)
}
