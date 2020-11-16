use super::*;
use mon_oeil_db as db;

impl From<db::DbError> for Error {
    fn from(err: db::DbError) -> Error {
        Error::Bug(format!("{:?}", err))
    }
}

impl From<db::User> for User {
    fn from(user: db::User) -> User {
        let db::User { username, password } = user;
        User { username, password }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_user() {
        let user = db::User {
            username: "user".to_owned(),
            password: "password".to_owned(),
        };
        assert_eq!(
            User {
                username: "user".to_owned(),
                password: "password".to_owned(),
            },
            User::from(user)
        )
    }
}
