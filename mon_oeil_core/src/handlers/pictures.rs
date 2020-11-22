use crate::{models::*, Error};
use mon_oeil_auth_shared::valid_jwt_admin;
use mon_oeil_db as db;
use mon_oeil_storage::*;

pub async fn post_picture(
    db: &db::GestureClientPool,
    storage: &Storage,
    id_gesture: &str,
    new_picture: NewPicture,
    content: Vec<u8>,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<String, Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;

    let format = new_picture.format.clone();
    valid_format(&format)?;

    let client = db.get().await.map_err(Error::from)?;
    let new_id = client.add_picture(new_picture.into(), &id_gesture).await?;

    storage.upload(&new_id, content, &format).await?;

    Ok(new_id)
}

pub async fn put_picture_file(
    db: &db::GestureClientPool,
    storage: &Storage,
    id: &str,
    new_picture_file_info: NewPictureFileInfo,
    content: Vec<u8>,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;

    let format = new_picture_file_info.format.clone();
    valid_format(&format)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .update_picture_format(&id, new_picture_file_info.into())
        .await?;

    storage.upload(&id, content, &format).await?;

    Ok(())
}

fn valid_format(format: &str) -> Result<(), Error> {
    if format != "png" && format != "jpg" && format != "jpeg" {
        Err(Error::NotAccepted(format!(
            "File format {} not accepted, we use only JPEG and PNG",
            format
        )))
    } else {
        Ok(())
    }
}

pub async fn put_picture_meta(
    db: &db::GestureClientPool,
    id: &str,
    new_picture_meta: NewPictureMeta,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(&hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .update_picture_meta(id, new_picture_meta.into())
        .await
        .map_err(Error::from)
}

pub async fn delete_picture(
    db: &db::GestureClientPool,
    storage: &Storage,
    id: &str,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;
    let client = db.get().await.map_err(Error::from)?;

    let format = client.get_picture_format(id).await?;

    storage.delete(&id, &format).await?;

    client.delete_picture(&id).await.map_err(Error::from)
}
