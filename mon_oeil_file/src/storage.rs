use async_std::prelude::*;
use bytes::Bytes;

const PICTURES_DIR: &str = "./pictures/";

#[cfg_attr(test, faux::create)]
pub struct Storage;

#[cfg_attr(test, faux::methods)]
impl Storage {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn save_picture(
        &self,
        filename: &str,
        bytes: Vec<Result<Bytes, ()>>, // try to do it with streams
    ) -> Result<(), StorageErr> {
        let filepath = format!("{}{}", PICTURES_DIR, sanitize_filename::sanitize(&filename));
        let mut f = async_std::fs::File::create(filepath)
            .await
            .map_err(|err| StorageErr(format!("{:?}", err)))?;

        // Field in turn is stream of *Bytes* object
        for chunk in bytes {
            let data = chunk.map_err(|err| StorageErr(format!("{:?}", err)))?;

            f.write_all(&data)
                .await
                .map_err(|err| StorageErr(format!("{:?}", err)))?;
        }

        Ok(())
    }
    pub async fn delete_picture(&self, id: &str) -> Result<(), StorageErr> {
        async_std::fs::remove_file(format!("{}{}{}", PICTURES_DIR, &id, ".png"))
            .await
            .map_err(|_| StorageErr("Delete file fail".to_owned()))
            .map(|_| ())
    }
}

#[derive(Debug)]
pub struct StorageErr(pub String);

#[cfg(test)]
mod tests {
    use super::*;

    mod integration {
        use super::*;
        use tokio_test::block_on;
        #[test]
        fn save_picture() {
            block_on(async {
                clear().await;
                let s = Storage::new();
                let to_copy = async_std::fs::read("./asset/dummy.png")
                    .await
                    .map_err(|_| ())
                    .map(|v| Bytes::from(v));

                let to_copy_save = to_copy.clone();

                let filename = "test.png";
                s.save_picture("test.png", vec![to_copy]).await.unwrap();

                let res = async_std::fs::read(format!("{}{}", PICTURES_DIR, filename))
                    .await
                    .map_err(|_| ())
                    .map(|v| Bytes::from(v));

                assert_eq!(to_copy_save, res)
            })
        }

        #[test]
        fn create_then_delete_picture() {
            block_on(async {
                clear().await;
                let s = Storage::new();
                let to_copy = async_std::fs::read("./asset/dummy.png")
                    .await
                    .map_err(|_| ())
                    .map(|v| Bytes::from(v));

                let to_copy_save = to_copy.clone();

                let filename = "test.png";
                s.save_picture("test.png", vec![to_copy]).await.unwrap();

                let res = async_std::fs::read(format!("{}{}", PICTURES_DIR, filename))
                    .await
                    .map_err(|_| ())
                    .map(|v| Bytes::from(v));

                assert_eq!(to_copy_save, res);

                s.delete_picture("test").await.unwrap();
                let res = async_std::fs::read(format!("{}{}", PICTURES_DIR, filename)).await;
                match res {
                    Err(_) => (),
                    _ => panic!("Delete failed"),
                }
            })
        }

        async fn clear() {
            match async_std::fs::remove_dir_all(PICTURES_DIR).await {
                _ => (),
            };
            async_std::fs::create_dir(PICTURES_DIR).await.unwrap();
        }
    }
}
