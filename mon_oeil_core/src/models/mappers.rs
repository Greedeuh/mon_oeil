use log::error;

use super::*;
use mon_oeil_auth_shared as auth;
use mon_oeil_db as db;
use mon_oeil_storage as storage;

impl From<db::Description> for Description {
    fn from(item: db::Description) -> Self {
        let db::Description {
            id,
            value,
            langs,
            meanings,
        } = item;
        Self {
            id,
            value,
            langs,
            meanings: meanings.into_iter().map(From::from).collect(),
        }
    }
}

impl From<db::Meaning> for Meaning {
    fn from(item: db::Meaning) -> Self {
        let db::Meaning { id, value, langs } = item;
        Self { id, value, langs }
    }
}

impl Picture {
    pub fn from(picture_db: db::Picture, storage_url: String) -> Self {
        let db::Picture { id, langs, .. } = picture_db;
        Self {
            id,
            langs,
            url: storage_url,
        }
    }
}

impl Into<db::NewGesture> for NewGesture {
    fn into(self) -> db::NewGesture {
        let Self { tags } = self;
        db::NewGesture { tags }
    }
}

impl Into<db::NewDescription> for NewDescription {
    fn into(self) -> db::NewDescription {
        let Self { value, langs } = self;
        db::NewDescription { value, langs }
    }
}

impl Into<db::NewMeaning> for NewMeaning {
    fn into(self) -> db::NewMeaning {
        let Self { value, langs } = self;
        db::NewMeaning { value, langs }
    }
}

impl Into<db::NewPicture> for NewPicture {
    fn into(self) -> db::NewPicture {
        let Self { langs, format } = self;
        db::NewPicture { langs, format }
    }
}

impl Into<db::NewPictureMeta> for NewPictureMeta {
    fn into(self) -> db::NewPictureMeta {
        let Self { langs } = self;
        db::NewPictureMeta { langs }
    }
}

impl Into<db::NewPictureFileInfo> for NewPictureFileInfo {
    fn into(self) -> db::NewPictureFileInfo {
        let Self { format } = self;
        db::NewPictureFileInfo { format }
    }
}

impl From<db::DbError> for Error {
    fn from(err: db::DbError) -> Error {
        match err {
            db::DbError::NotFound => Error::NotFound,
            db::DbError::ForeignKeyViolation(err) => {
                error!("{:?}", err);
                Error::NotFound
            }
            db::DbError::Other(err) => Error::Bug(format!("{:?}", err)),
        }
    }
}

impl From<auth::JwtValidationError> for Error {
    fn from(_err: auth::JwtValidationError) -> Error {
        Error::Auth
    }
}

impl From<storage::StorageError> for Error {
    fn from(err: storage::StorageError) -> Error {
        Error::Bug(format!("{:?}", err))
    }
}
