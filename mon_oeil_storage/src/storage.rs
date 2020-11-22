// TODO
#![allow(dead_code)]

use cloud_storage::Object;
#[cfg(feature = "mock")]
use mockall::automock;

use crate::models::*;

const URL_GCS_READ: &str = "https://storage.googleapis.com";

#[derive(Clone)]
pub struct Storage {
    pub bucket_name: String,
}

#[cfg_attr(feature = "mock", automock)]
impl Storage {
    pub fn new(bucket_name: &str) -> Self {
        Self {
            bucket_name: bucket_name.to_owned(),
        }
    }

    pub async fn upload(
        &self,
        id: &str,
        content: Vec<u8>,
        img_type: &str,
    ) -> Result<(), StorageError> {
        Object::create(
            &self.bucket_name,
            content,
            &format!("{}.{}", id, img_type),
            &format!("image/{}", img_type),
        )
        .await?;

        Ok(())
    }

    pub async fn delete(&self, id: &str, img_type: &str) -> Result<(), StorageError> {
        Object::delete(&self.bucket_name, &format!("{}.{}", id, img_type)).await?;

        Ok(())
    }

    pub fn get_url(&self, id: &str, img_type: &str) -> String {
        format!("{}/{}/{}.{}", URL_GCS_READ, self.bucket_name, id, img_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_url() {
        assert_eq!(
            Storage::new("mon_oeil_pictures_test").get_url("test", "png"),
            "https://storage.googleapis.com/mon_oeil_pictures_test/test.png".to_owned()
        )
    }

    #[tokio::test]
    async fn upload_and_delete_on_real_storage_work() {
        let storage = Storage::new("mon_oeil_pictures_test");

        let file = std::fs::read("asset/dummy.png").unwrap();

        let res_upload = storage.upload("test", file.clone(), "png").await.is_ok();
        let res_delete = storage.delete("test", "png").await.is_ok();

        // assert at the end to have more chance to stay clean in our test storage
        assert!(res_upload);
        assert!(res_delete);
    }
}
