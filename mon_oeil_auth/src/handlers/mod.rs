use blake2::{Blake2b, Digest};
use chrono::prelude::*;

use crate::models::*;
use mon_oeil_auth_shared::*;
use mon_oeil_db as db;

pub async fn login(
    credential: &Credentials,
    hs256_private_key: &str,
    salt_hash: &str,
    db: &db::GestureClientPool,
) -> Result<String, Error> {
    let client = db.get().await.map_err(Error::from)?;

    let user = match client.get_user(&credential.username).await {
        Ok(Some(u)) => u,
        Ok(None) => return Err(Error::Auth),
        e => return Err(Error::Bug(format!("{:?}", e))),
    };

    let hash = Blake2b::new()
        .chain(&credential.password)
        .chain(salt_hash)
        .finalize();
    println!("{} {}", &credential.password, salt_hash);
    let hash_password = dbg!(format!("{:x}", hash));

    if user.password != hash_password {
        return Err(Error::Auth);
    }

    let expire = Utc::now()
        .checked_add_signed(chrono::Duration::weeks(1000))
        .ok_or(Error::Auth)?;
    encode_jwt(
        hs256_private_key,
        JwtPayload {
            level: Level::Admin,
            exp: expire.timestamp(),
        },
    )
    .map_err(|e| Error::Bug(format!("{:?}", e)))
}
