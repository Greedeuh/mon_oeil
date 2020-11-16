mod mappers;

#[derive(PartialEq, Eq, Debug)]
pub enum StorageError {
    Other(String),
}
