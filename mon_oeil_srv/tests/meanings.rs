#[macro_use]
extern crate serial_test;
use actix_web::http::StatusCode;
use regex::Regex;

mod utils;

use mon_oeil_core::*;
use utils::setup;

#[actix_rt::test]
#[serial]
async fn post_description_meaning_should_reject_unauth() {
    setup::reset_db();
    setup::insert_gesture_with_description();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un petit meaning".to_owned(),
        langs: vec!["fr".to_owned(), "us".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/descriptions/2ae70884-97bd-401d-8f43-d1778d4502d2/meanings",
            address
        ))
        .json(&new_meaning)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn post_gesture_meaning_should_reject_unauth() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un petit meaning".to_owned(),
        langs: vec!["fr".to_owned(), "us".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/meanings",
            address
        ))
        .json(&new_meaning)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn post_description_meaning_should_accept_auth() {
    setup::reset_db();
    setup::insert_gesture_with_description();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un petit meaning".to_owned(),
        langs: vec!["fr".to_owned(), "us".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/descriptions/2ae70884-97bd-401d-8f43-d1778d4502d2/meanings",
            address
        ))
        .json(&new_meaning)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();
    assert_ne!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn post_gestures_meaning_should_accept_auth() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un petit meaning".to_owned(),
        langs: vec!["fr".to_owned(), "us".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/meanings",
            address
        ))
        .json(&new_meaning)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_ne!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn post_desctipions_meaning_on_non_existing_gesture_should_fail() {
    setup::reset_db();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un petit meaning".to_owned(),
        langs: vec!["fr".to_owned(), "us".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/descriptions/2ae70884-97bd-401d-8f43-d1778d4502d2/meanings",
            address
        ))
        .json(&new_meaning)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
#[serial]
async fn post_gesture_meaning_on_non_existing_gesture_should_fail() {
    setup::reset_db();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un petit meaning".to_owned(),
        langs: vec!["fr".to_owned(), "us".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/meanings",
            address
        ))
        .json(&new_meaning)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
#[serial]
async fn post_description_meaning_should_return_new_uuid() {
    setup::reset_db();
    setup::insert_gesture_with_description();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un petit meaning".to_owned(),
        langs: vec!["fr".to_owned(), "us".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/descriptions/2ae70884-97bd-401d-8f43-d1778d4502d2/meanings",
            address
        ))
        .json(&new_meaning)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    let uuid = res.text().await.unwrap();
    let uuid = uuid.replace("\"", "");

    let uuid_regex =
        Regex::new(r"^[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}$").unwrap();
    assert!(uuid_regex.is_match(&uuid));
}

#[actix_rt::test]
#[serial]
async fn post_gesture_meaning_should_return_new_uuid() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un petit meaning".to_owned(),
        langs: vec!["fr".to_owned(), "us".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/meanings",
            address
        ))
        .json(&new_meaning)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let uuid = res.text().await.unwrap();
    let uuid = uuid.replace("\"", "");

    let uuid_regex =
        Regex::new(r"^[0-9a-f]{8}\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\b[0-9a-f]{12}$").unwrap();
    assert!(uuid_regex.is_match(&uuid));
}

#[actix_rt::test]
#[serial]
async fn post_decription_meaning_should_return_uuid_random() {
    setup::reset_db();
    setup::insert_gesture_with_description();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un petit meaning".to_owned(),
        langs: vec!["fr".to_owned(), "us".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/descriptions/2ae70884-97bd-401d-8f43-d1778d4502d2/meanings",
            address
        ))
        .json(&new_meaning)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::CREATED);

    let uuid = res.text().await.unwrap();
    let uuid1 = uuid.replace("\"", "");

    let res = client
        .post(&format!(
            "{}/descriptions/2ae70884-97bd-401d-8f43-d1778d4502d2/meanings",
            address
        ))
        .json(&new_meaning)
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
async fn post_gesture_meaning_should_return_uuid_random() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un petit meaning".to_owned(),
        langs: vec!["fr".to_owned(), "us".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/meanings",
            address
        ))
        .json(&new_meaning)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let uuid = res.text().await.unwrap();
    let uuid1 = uuid.replace("\"", "");

    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/meanings",
            address
        ))
        .json(&new_meaning)
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
async fn get_gestures_after_post_description_meaning_should_return_gesture_with_posted_meaning() {
    setup::reset_db();
    setup::insert_gesture_with_description();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un petit meaning".to_owned(),
        langs: vec!["fr".to_owned(), "us".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/descriptions/2ae70884-97bd-401d-8f43-d1778d4502d2/meanings",
            address
        ))
        .json(&new_meaning)
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

    let tags = vec!["tag1".to_owned(), "tag2".to_owned()];
    assert_eq!(
        vec![Gesture {
            id: "ce27c124-e47b-490f-b8fe-3f37d5dbbef6".to_owned(),
            tags,
            descriptions: vec![Description {
                id: "2ae70884-97bd-401d-8f43-d1778d4502d2".to_owned(),
                value: "Une petite description".to_owned(),
                langs: vec!["fr".to_owned(), "us".to_owned()],
                meanings: vec![Meaning {
                    id: uuid,
                    value: "Un petit meaning".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                }]
            }],
            meanings: vec![],
            pictures: vec![]
        }],
        gestures
    )
}

#[actix_rt::test]
#[serial]
async fn get_gestures_after_post_gestures_meaning_should_return_gesture_with_posted_meaning() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un petit meaning".to_owned(),
        langs: vec!["fr".to_owned(), "us".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6/meanings",
            address
        ))
        .json(&new_meaning)
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

    let tags = vec!["tag1".to_owned(), "tag2".to_owned()];
    assert_eq!(
        vec![Gesture {
            id: "ce27c124-e47b-490f-b8fe-3f37d5dbbef6".to_owned(),
            tags,
            descriptions: vec![],
            meanings: vec![Meaning {
                id: uuid,
                value: "Un petit meaning".to_owned(),
                langs: vec!["fr".to_owned(), "us".to_owned()],
            }],
            pictures: vec![]
        }],
        gestures
    )
}

#[actix_rt::test]
#[serial]
async fn delete_meaning_should_reject_unauth() {
    setup::reset_db();
    setup::insert_gesture_with_description_with_meaning();

    let address = setup::spawn_app();

    let client = reqwest::Client::new();
    let res = client
        .delete(&format!(
            "{}/meanings/e2c6eee0-49a7-49c4-9a0f-a9c6e6f668d8",
            address
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn delete_meaning_should_accept_auth() {
    setup::reset_db();
    setup::insert_gesture_with_description_with_meaning();

    let address = setup::spawn_app();

    let client = reqwest::Client::new();
    let res = client
        .delete(&format!(
            "{}/meanings/e2c6eee0-49a7-49c4-9a0f-a9c6e6f668d8",
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
async fn delete_not_existing_meaning_should_fail() {
    setup::reset_db();

    let address = setup::spawn_app();

    let client = reqwest::Client::new();
    let res = client
        .delete(&format!(
            "{}/meanings/e2c6eee0-49a7-49c4-9a0f-a9c6e6f668d8",
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
async fn get_gestures_after_delete_description_meaning_should_return_empty_description() {
    setup::reset_db();
    setup::insert_gesture_with_description_with_meaning();

    let address = setup::spawn_app();

    let client = reqwest::Client::new();

    let res = client
        .delete(&format!(
            "{}/meanings/e2c6eee0-49a7-49c4-9a0f-a9c6e6f668d8",
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
            descriptions: vec![Description {
                id: "2ae70884-97bd-401d-8f43-d1778d4502d2".to_owned(),
                value: "Une petite description".to_owned(),
                langs: vec!["fr".to_owned(), "us".to_owned()],
                meanings: vec![]
            }],
            meanings: vec![],
            pictures: vec![]
        }],
        gestures
    )
}

#[actix_rt::test]
#[serial]
async fn get_gestures_after_delete_gesture_meaning_should_return_empty_gesture() {
    setup::reset_db();
    setup::insert_gesture_with_meaning();

    let address = setup::spawn_app();

    let client = reqwest::Client::new();

    let res = client
        .delete(&format!(
            "{}/meanings/59c25147-021e-4584-9c35-97cbf060cc89",
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
async fn put_meaning_should_reject_unauth() {
    setup::reset_db();
    setup::insert_gesture_with_meaning();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un autre".to_owned(),
        langs: vec!["kr".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .put(&format!(
            "{}/meanings/59c25147-021e-4584-9c35-97cbf060cc89",
            address
        ))
        .json(&new_meaning)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn put_meaning_should_accept_auth() {
    setup::reset_db();
    setup::insert_gesture_with_meaning();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un autre".to_owned(),
        langs: vec!["kr".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .put(&format!(
            "{}/meanings/59c25147-021e-4584-9c35-97cbf060cc89",
            address
        ))
        .json(&new_meaning)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success());
}

#[actix_rt::test]
#[serial]
async fn put_not_existing_meaning_should_fail() {
    setup::reset_db();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un autre".to_owned(),
        langs: vec!["kr".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .put(&format!(
            "{}/meanings/59c25147-021e-4584-9c35-97cbf060cc89",
            address
        ))
        .json(&new_meaning)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
#[serial]
async fn get_gesture_after_put_meaning_should_return_gesture_with_updated_meaning() {
    setup::reset_db();
    setup::insert_gesture_with_meaning();

    let address = setup::spawn_app();

    let new_meaning = NewMeaning {
        value: "Un autre".to_owned(),
        langs: vec!["kr".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .put(&format!(
            "{}/meanings/59c25147-021e-4584-9c35-97cbf060cc89",
            address
        ))
        .json(&new_meaning)
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
            meanings: vec![Meaning {
                id: "59c25147-021e-4584-9c35-97cbf060cc89".to_owned(),
                value: "Un autre".to_owned(),
                langs: vec!["kr".to_owned()],
            }],
            pictures: vec![]
        }],
        gestures
    )
}
