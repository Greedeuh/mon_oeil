use crate::{db, models::*, Error};
use mon_oeil_auth_shared::valid_jwt_admin;

pub async fn post_gesture_s_meaning(
    db: &db::DbPool,
    id_gesture: &str,
    meaning: NewMeaning,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .add_meaning(meaning, Some(&id_gesture), None)
        .await
        .map_err(Error::from)
}

pub async fn post_description_s_meaning(
    db: &db::DbPool,
    id_descirption: &str,
    meaning: NewMeaning,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client
        .add_meaning(meaning, None, Some(&id_descirption))
        .await
        .map_err(Error::from)
}

pub async fn delete_meaning(
    db: &db::DbPool,
    id: &str,
    hs256_private_key: &str,
    jwt: &str,
) -> Result<(), Error> {
    valid_jwt_admin(hs256_private_key, jwt).map_err(Error::from)?;

    let client = db.get().await.map_err(Error::from)?;
    client.delete_meaning(&id).await.map_err(Error::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADMIN_TOKEN: &str = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjIyMDM0MDcxOTgsImxldmVsIjoiQWRtaW4ifQ.RLE2du-ICZ0mlFl02YytZC02Xk0U5qyNRBxhi_-SvY8";
    const HS256_PRIVATE_KEY: &str = "private_key";

    #[tokio::test]
    async fn add_gesture_s_meaning() {
        let mut pool = db::DbPool::faux();
        let mut client = db::DbClient::faux();
        unsafe {
            faux::when!(client.add_meaning).then(|_| Ok(()));
        }
        faux::when!(pool.get).once().safe_then(move |_| Ok(client));

        let new_meaning = NewMeaning {
            value: "Un petit meaning".to_owned(),
            langs: vec!["fr".to_owned(), "us".to_owned()],
        };
        let res = post_gesture_s_meaning(&pool, "id", new_meaning, HS256_PRIVATE_KEY, ADMIN_TOKEN);
        assert!(res.await.is_ok());
    }

    #[tokio::test]
    async fn add_decription_s_meaning() {
        let mut pool = db::DbPool::faux();
        let mut client = db::DbClient::faux();
        unsafe {
            faux::when!(client.add_meaning).then(|_| Ok(()));
        }
        faux::when!(pool.get).once().safe_then(move |_| Ok(client));

        let new_meaning = NewMeaning {
            value: "Un petit meaning".to_owned(),
            langs: vec!["fr".to_owned(), "us".to_owned()],
        };
        let res =
            post_description_s_meaning(&pool, "id", new_meaning, HS256_PRIVATE_KEY, ADMIN_TOKEN);
        assert!(res.await.is_ok());
    }

    #[tokio::test]
    async fn delete_meaning() {
        let mut pool = db::DbPool::faux();
        let mut client = db::DbClient::faux();
        unsafe {
            // TODO : Comprendre quel est la différene avec le test post du dessus
            faux::when!(client.delete_meaning).then(|_| Ok(()));
        }
        faux::when!(pool.get).once().safe_then(move |_| Ok(client));

        let res = super::delete_meaning(&pool, "id", HS256_PRIVATE_KEY, ADMIN_TOKEN);
        assert!(res.await.is_ok());
    }
}
