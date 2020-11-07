use crate::{db, models::*, Error};
use mon_oeil_auth_shared::valid_jwt_admin;

pub async fn post_picture_meta(
    db: &db::DbPool,
    id_gesture: &str,
    picture: NewPicture,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .add_picture(picture, &id_gesture)
        .await
        .map_err(Error::from)
}

pub async fn delete_picture(
    db: &db::DbPool,
    id: &str,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;
    let client = db.get().await.map_err(Error::from)?;
    client.delete_picture(&id).await.map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADMIN_TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjIyMDM0MDcxOTgsImxldmVsIjoiQWRtaW4ifQ.RLE2du-ICZ0mlFl02YytZC02Xk0U5qyNRBxhi_-SvY8";
    const HS256_PRIVATE_KEY: &str = "private_key";

    #[tokio::test]
    async fn add_picture() {
        let mut pool = db::DbPool::faux();
        let mut client = db::DbClient::faux();
        unsafe {
            faux::when!(client.add_picture).then(|_| Ok(()));
        }
        faux::when!(pool.get).once().safe_then(move |_| Ok(client));

        let new_picture = NewPicture {
            langs: vec!["fr".to_owned(), "us".to_owned()],
        };
        let res = post_picture_meta(&pool, "id", new_picture, HS256_PRIVATE_KEY, ADMIN_TOKEN).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn delete_picture() {
        let mut pool = db::DbPool::faux();
        let mut client = db::DbClient::faux();
        unsafe {
            // TODO : Comprendre quel est la différene avec le test post du dessus
            faux::when!(client.delete_picture).then(|_| Ok(()));
        }
        faux::when!(pool.get).once().safe_then(move |_| Ok(client));

        let res = super::delete_picture(&pool, "id", HS256_PRIVATE_KEY, ADMIN_TOKEN).await;
        println!("{:?}", res);
        assert!(res.is_ok());
    }
}
