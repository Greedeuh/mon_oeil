#[macro_use]
extern crate serial_test;
use actix_web::http::StatusCode;
use regex::Regex;

mod utils;

use mon_oeil_core::*;
use mon_oeil_storage::*;
use reqwest::multipart;
use utils::check;
use utils::setup;

#[actix_rt::test]
#[serial]
async fn post_picture_should_reject_unauth() {
    setup::reset_db();

    let address = setup::spawn_app();

    let mut storage = Storage::default();
    storage.expect_upload().returning(|_, _, _| Ok(()));

    let file = std::fs::read("asset/dummy.png").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/pictures?langs=fr;us",
            address
        ))
        .multipart(form)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn post_picture_should_accept_auth() {
    setup::reset_db();

    let address = setup::spawn_app();

    let mut storage = Storage::default();
    storage.expect_upload().returning(|_, _, _| Ok(()));

    let file = std::fs::read("asset/dummy.png").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/pictures?langs=fr;us",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_ne!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn post_picture_should_reject_no_file() {
    setup::reset_db();

    let address = setup::spawn_app();

    let mut storage = Storage::default();
    storage.expect_upload().returning(|_, _, _| Ok(()));

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/pictures?langs=fr;us",
            address
        ))
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
#[serial]
async fn post_picture_on_non_existing_gesture_should_fail() {
    setup::reset_db();

    let address = setup::spawn_app();

    let mut storage = Storage::default();
    storage.expect_upload().returning(|_, _, _| Ok(()));

    let file = std::fs::read("asset/dummy.png").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/pictures?langs=fr;us",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
#[serial]
async fn post_picture_without_langs_is_ok() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage.expect_upload().returning(|_, _, _| Ok(()));

        storage
    });

    let file = std::fs::read("asset/dummy.png").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/pictures",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
}

#[actix_rt::test]
#[serial]
async fn post_picture_accept_png() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage.expect_upload().returning(|_, _, _| Ok(()));

        storage
    });

    let file = std::fs::read("asset/dummy.png").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/pictures?langs=fr;us",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
}

#[actix_rt::test]
#[serial]
async fn post_picture_should_accept_jpg() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage.expect_upload().returning(|_, _, _| Ok(()));

        storage
    });

    let file = std::fs::read("asset/dummy.jpg").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.jpg")
            .mime_str("image/jpg")
            .unwrap(),
    );
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/pictures?langs=fr;us",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
}

#[actix_rt::test]
#[serial]
async fn post_picture_should_reject_txt() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage.expect_upload().returning(|_, _, _| Ok(()));

        storage
    });

    let file = std::fs::read("asset/dummy.txt").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.txt")
            .mime_str("text/plain")
            .unwrap(),
    );
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/pictures?langs=fr;us",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[actix_rt::test]
#[serial]
async fn post_picture_should_return_new_uuid() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage.expect_upload().returning(|_, _, _| Ok(()));

        storage
    });

    let file = std::fs::read("asset/dummy.png").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/pictures?langs=fr;us",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let uuid = res.text().await.unwrap();
    let uuid = uuid.replace("\"", "");

    let uuid_regex =
        Regex::new(r"^[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}$").unwrap();
    debug_assert!(uuid_regex.is_match(&uuid));
}

#[actix_rt::test]
#[serial]
async fn post_gestures_should_return_uuid_random() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage.expect_upload().returning(|_, _, _| Ok(()));

        storage
    });

    let file = std::fs::read("asset/dummy.png").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/pictures?langs=fr;us",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    let uuid = res.text().await.unwrap();
    let uuid1 = uuid.replace("\"", "");

    let file = std::fs::read("asset/dummy.png").unwrap();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/pictures?langs=fr;us",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    let uuid = res.text().await.unwrap();
    let uuid2 = uuid.replace("\"", "");

    assert_ne!(uuid1, uuid2)
}

#[actix_rt::test]
#[serial]
async fn get_gestures_after_post_picture_should_return_gesture_with_posted_picture() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage.expect_upload().returning(|_, _, _| Ok(()));
        storage
            .expect_get_url()
            .returning(|id, fmt| format!("http://monoielfakeapp.com/{}.{}", id, fmt));
        storage
    });

    let file = std::fs::read("asset/dummy.png").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/pictures?langs=fr;us",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    let uuid = res.text().await.unwrap();
    let uuid = uuid.replace("\"", "");

    let res = client
        .get(&format!("{}/gestures", address))
        .send()
        .await
        .unwrap();

    let gestures: Vec<Gesture> = res.json().await.unwrap();

    assert_eq!(
        gestures,
        vec![Gesture {
            id: "ce27c124-e47b-490f-b8fe-3f37d5dbbef6".to_owned(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            descriptions: vec![],
            meanings: vec![],
            pictures: vec![Picture {
                id: uuid.clone(),
                langs: vec!["fr".to_owned(), "us".to_owned()],
                url: format!("http://monoielfakeapp.com/{}.png", uuid),
            }]
        }]
    )
}

#[actix_rt::test]
#[serial]
async fn delete_picture_should_reject_unauth() {
    setup::reset_db();
    setup::insert_gesture_with_picture();

    let address = setup::spawn_app();

    let client = reqwest::Client::new();
    let res = client
        .delete(&format!(
            "{}/pictures/283e7b04-7c13-4154-aafe-8e55b6960fe3",
            address
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn delete_picture_should_accept_auth() {
    setup::reset_db();
    setup::insert_gesture_with_picture();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage.expect_delete().returning(|_| Ok(()));

        storage
    });

    let client = reqwest::Client::new();
    let res = client
        .delete(&format!(
            "{}/pictures/283e7b04-7c13-4154-aafe-8e55b6960fe3",
            address
        ))
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
}

#[actix_rt::test]
#[serial]
async fn delete_not_existing_picture_should_fail() {
    setup::reset_db();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage.expect_delete().returning(|_| Ok(()));

        storage
    });

    let client = reqwest::Client::new();
    let res = client
        .delete(&format!(
            "{}/pictures/283e7b04-7c13-4154-aafe-8e55b6960fe3",
            address
        ))
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
#[serial]
async fn get_gestures_after_delete_picture_should_return_nothing() {
    setup::reset_db();
    setup::insert_gesture_with_picture();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage.expect_delete().returning(|_| Ok(()));

        storage
    });

    let client = reqwest::Client::new();

    let res = client
        .delete(&format!(
            "{}/pictures/283e7b04-7c13-4154-aafe-8e55b6960fe3",
            address
        ))
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());

    let res = client
        .get(&format!("{}/gestures", address))
        .send()
        .await
        .unwrap();
    let gestures: Vec<Gesture> = res.json().await.unwrap();

    let tags = vec!["tag1".to_owned(), "tag2".to_owned()];

    assert_eq!(
        vec![Gesture {
            id: "ce27c124-e47b-490f-b8fe-3f37d5dbbef6".to_owned(),
            tags,
            descriptions: vec![],
            meanings: vec![],
            pictures: vec![]
        }],
        gestures
    )
}

#[actix_rt::test]
#[serial]
async fn put_picture_meta_should_reject_unauth() {
    setup::reset_db();
    setup::insert_gesture_with_meaning();

    let address = setup::spawn_app();

    let new_picture_meta = NewPictureMeta {
        langs: vec!["kr".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .put(&format!(
            "{}/pictures/283e7b04-7c13-4154-aafe-8e55b6960fe3/meta",
            address
        ))
        .json(&new_picture_meta)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn put_picture_meta_should_accept_auth() {
    setup::reset_db();
    setup::insert_gesture_with_picture();

    let address = setup::spawn_app();

    let new_picture_meta = NewPictureMeta {
        langs: vec!["kr".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .put(&format!(
            "{}/pictures/283e7b04-7c13-4154-aafe-8e55b6960fe3/meta",
            address
        ))
        .json(&new_picture_meta)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success());
}

#[actix_rt::test]
#[serial]
async fn put_picture_meta_on_not_existing_picture_should_fail() {
    setup::reset_db();

    let address = setup::spawn_app();

    let new_picture_meta = NewPictureMeta {
        langs: vec!["kr".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .put(&format!(
            "{}/pictures/283e7b04-7c13-4154-aafe-8e55b6960fe3/meta",
            address
        ))
        .json(&new_picture_meta)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
#[serial]
async fn get_gesture_after_put_picture_meta_should_return_gesture_with_updated_picture() {
    setup::reset_db();
    setup::insert_gesture_with_picture();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage
            .expect_get_url()
            .returning(|id, fmt| format!("http://monoielfakeapp.com/{}.{}", id, fmt));

        storage
    });

    let new_picture_meta = NewPictureMeta {
        langs: vec!["kr".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .put(&format!(
            "{}/pictures/283e7b04-7c13-4154-aafe-8e55b6960fe3/meta",
            address
        ))
        .json(&new_picture_meta)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success());

    let res = client
        .get(&format!("{}/gestures", address))
        .send()
        .await
        .unwrap();

    let gestures: Vec<Gesture> = res.json().await.unwrap();

    assert_eq!(
        vec![Gesture {
            id: "ce27c124-e47b-490f-b8fe-3f37d5dbbef6".to_owned(),
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
            descriptions: vec![],
            meanings: vec![],
            pictures: vec![Picture {
                id: "283e7b04-7c13-4154-aafe-8e55b6960fe3".to_owned(),
                langs: vec!["kr".to_owned()],
                url: "http://monoielfakeapp.com/283e7b04-7c13-4154-aafe-8e55b6960fe3.png"
                    .to_owned(),
            }]
        }],
        gestures
    )
}

#[actix_rt::test]
#[serial]
async fn put_picture_file_should_reject_unauth() {
    setup::reset_db();
    setup::insert_gesture_with_meaning();

    let address = setup::spawn_app();

    let file = std::fs::read("asset/dummy.png").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let res = client
        .put(&format!(
            "{}/pictures/283e7b04-7c13-4154-aafe-8e55b6960fe3/file",
            address
        ))
        .multipart(form)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn put_picture_file_should_accept_auth() {
    setup::reset_db();
    setup::insert_gesture_with_picture();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage.expect_upload().returning(|_, _, _| Ok(()));

        storage
    });

    let file = std::fs::read("asset/dummy.png").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let res = client
        .put(&format!(
            "{}/pictures/283e7b04-7c13-4154-aafe-8e55b6960fe3/file",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success());
}

#[actix_rt::test]
#[serial]
async fn put_picture_file_on_not_existing_picture_should_fail() {
    setup::reset_db();

    let address = setup::spawn_app();

    let file = std::fs::read("asset/dummy.png").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.png")
            .mime_str("image/png")
            .unwrap(),
    );
    let res = client
        .put(&format!(
            "{}/pictures/283e7b04-7c13-4154-aafe-8e55b6960fe3/file",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
#[serial]
async fn put_picture_file_with_other_format_should_change_format() {
    setup::reset_db();
    setup::insert_gesture_with_picture();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage.expect_upload().returning(|_, _, _| Ok(()));

        storage
    });

    let file = std::fs::read("asset/dummy.jpg").unwrap();

    let client = reqwest::Client::new();
    let form = multipart::Form::new().part(
        "picture",
        multipart::Part::bytes(file)
            .file_name("dummy.jpg")
            .mime_str("image/jpg")
            .unwrap(),
    );
    let res = client
        .put(&format!(
            "{}/pictures/283e7b04-7c13-4154-aafe-8e55b6960fe3/file",
            address
        ))
        .multipart(form)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success());

    let row_picture = check::select_picture();
    let format: String = row_picture.get("format");
    assert_eq!(format, "jpg".to_owned());
}
