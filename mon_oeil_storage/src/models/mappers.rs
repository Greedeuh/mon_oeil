use super::*;
use cloud_storage::Error;

impl From<Error> for StorageError {
    fn from(err: Error) -> Self {
        Self::Other(format!("{:?}", err))
    }
}
