use chrono::prelude::*;

use crate::models::*;
use mon_oeil_auth_shared::*;
use mon_oeil_db as db;

pub async fn login(
    credential: &Credentials,
    hs256_private_key: &str,
    db: &db::GestureClientPool,
) -> Result<String, Error> {
    let client = db.get().await.map_err(Error::from)?;

    let user = match client.get_user(&credential.username).await {
        Ok(Some(u)) => u,
        Ok(None) => return Err(Error::Auth),
        e => return Err(Error::Bug(format!("{:?}", e))),
    };

    if user.password != credential.password {
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
