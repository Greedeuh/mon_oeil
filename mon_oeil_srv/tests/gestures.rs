#[macro_use]
extern crate serial_test;
use actix_web::http::StatusCode;
use regex::Regex;

mod utils;

use mon_oeil_core::*;
use mon_oeil_storage::*;
use utils::setup;

#[actix_rt::test]
#[serial]
async fn post_gesture_should_reject_unauth() {
    setup::reset_db();

    let address = setup::spawn_app();

    let new_gesture = NewGesture {
        tags: vec!["tag1".to_owned(), "tag2".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!("{}/gestures", address))
        .json(&new_gesture)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn post_gesture_should_accept_auth() {
    setup::reset_db();

    let address = setup::spawn_app();

    let new_gesture = NewGesture {
        tags: vec!["tag1".to_owned(), "tag2".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!("{}/gestures", address))
        .json(&new_gesture)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
}

#[actix_rt::test]
#[serial]
async fn post_gesture_should_return_new_uuid() {
    setup::reset_db();

    let address = setup::spawn_app();

    let new_gesture = NewGesture {
        tags: vec!["tag1".to_owned(), "tag2".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!("{}/gestures", address))
        .json(&new_gesture)
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
async fn post_gestures_should_return_uuid_random() {
    setup::reset_db();

    let address = setup::spawn_app();

    let new_gesture = NewGesture {
        tags: vec!["tag1".to_owned(), "tag2".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!("{}/gestures", address))
        .json(&new_gesture)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);

    let uuid = res.text().await.unwrap();
    let uuid1 = uuid.replace("\"", "");

    let res = client
        .post(&format!("{}/gestures", address))
        .json(&new_gesture)
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
async fn get_gestures_with_empty_db_should_return_empty() {
    setup::reset_db();

    let address = setup::spawn_app();

    let client = reqwest::Client::new();
    let res = client
        .get(&format!("{}/gestures", address))
        .send()
        .await
        .unwrap();

    let gestures: Vec<Gesture> = res.json().await.unwrap();
    assert_eq!(gestures, vec![])
}

#[actix_rt::test]
#[serial]
async fn get_gestures_after_post_gestures_should_return_posted_gesture() {
    setup::reset_db();

    let address = setup::spawn_app();

    let new_gesture = NewGesture {
        tags: vec!["tag1".to_owned(), "tag2".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!("{}/gestures", address))
        .json(&new_gesture)
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
        vec![Gesture {
            id: uuid,
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
            descriptions: vec![],
            meanings: vec![],
            pictures: vec![]
        }],
        gestures
    )
}

#[actix_rt::test]
#[serial]
async fn get_gestures_with_full_links() {
    setup::reset_db();
    setup::insert_2_gestures_with_full_links();

    let address = setup::spawn_app_with_storage(|| {
        let mut storage = Storage::default();
        storage
            .expect_get_url()
            .returning(|id, fmt| format!("http://monoielfakeapp.com/{}.{}", id, fmt));

        storage
    });

    let client = reqwest::Client::new();

    let res = client
        .get(&format!("{}/gestures", address))
        .send()
        .await
        .unwrap();

    let gestures: Vec<Gesture> = res.json().await.unwrap();

    assert_eq!(
        gestures,
        vec![
            Gesture {
                id: "ce27c124-e47b-490f-b8fe-3f37d5dbbef6".to_owned(),
                tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                descriptions: vec![
                    Description {
                        id: "2ae70884-97bd-401d-8f43-d1778d4502d2".to_owned(),
                        value: "Une petite description".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                        meanings: vec![
                            Meaning {
                                id: "e2c6eee0-49a7-49c4-9a0f-a9c6e6f668d8".to_owned(),
                                value: "Un petit meaning".to_owned(),
                                langs: vec!["fr".to_owned(), "us".to_owned()],
                            },
                            Meaning {
                                id: "45dca590-6bc4-4e4b-ad0c-0fe57a3a9643".to_owned(),
                                value: "Un petit meaning".to_owned(),
                                langs: vec!["fr".to_owned(), "us".to_owned()],
                            },
                        ],
                    },
                    Description {
                        id: "1c53f9ad-98b4-444c-9ec9-e8f92f1e5d28".to_owned(),
                        value: "Une petite description".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                        meanings: vec![],
                    },
                ],
                meanings: vec![
                    Meaning {
                        id: "59c25147-021e-4584-9c35-97cbf060cc89".to_owned(),
                        value: "Un petit meaning".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                    },
                    Meaning {
                        id: "02ca8fb9-c56e-4e45-b13e-98a6732f780a".to_owned(),
                        value: "Un petit meaning".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                    },
                ],
                pictures: vec![
                    Picture {
                        id: "283e7b04-7c13-4154-aafe-8e55b6960fe3".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                        url: "http://monoielfakeapp.com/283e7b04-7c13-4154-aafe-8e55b6960fe3.png"
                            .to_owned(),
                    },
                    Picture {
                        id: "03b9bfc6-fa22-4ffb-9464-93c1be842ace".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                        url: "http://monoielfakeapp.com/03b9bfc6-fa22-4ffb-9464-93c1be842ace.png"
                            .to_owned(),
                    },
                ],
            },
            Gesture {
                id: "16991982-1752-4aa0-bb22-db3fbceb3780".to_owned(),
                tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                descriptions: vec![Description {
                    id: "cdbcd8fb-3d6d-4f09-86ba-37a6ec1dd293".to_owned(),
                    value: "Une petite description".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                    meanings: vec![],
                }],
                meanings: vec![Meaning {
                    id: "4719b1d7-2810-4f7d-865d-03ee44cf0add".to_owned(),
                    value: "Un petit meaning".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                }],
                pictures: vec![Picture {
                    id: "6e1ee88d-fd97-488c-9aa8-6b66a3f3e714".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                    url: "http://monoielfakeapp.com/6e1ee88d-fd97-488c-9aa8-6b66a3f3e714.png"
                        .to_owned(),
                }],
            },
        ]
    )
}

#[actix_rt::test]
#[serial]
async fn put_gesture_should_reject_unauth() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app();

    let updatable_gesture = NewGesture {
        tags: vec!["tag1".to_owned(), "tag2".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .put(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6",
            address
        ))
        .json(&updatable_gesture)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn put_gesture_should_accept_auth() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app();

    let updatable_gesture = NewGesture {
        tags: vec!["tag3".to_owned(), "tag4".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .put(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6",
            address
        ))
        .json(&updatable_gesture)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success());
}

#[actix_rt::test]
#[serial]
async fn put_not_existing_gesture_should_fail() {
    setup::reset_db();

    let address = setup::spawn_app();

    let updatable_gesture = NewGesture {
        tags: vec!["tag3".to_owned(), "tag4".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .put(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6",
            address
        ))
        .json(&updatable_gesture)
        .header("Authorization", setup::ADMIN_TOKEN)
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[actix_rt::test]
#[serial]
async fn get_gesture_after_put_should_return_updated_gesture() {
    setup::reset_db();
    setup::insert_gesture_without_links();

    let address = setup::spawn_app();

    let updatable_gesture = NewGesture {
        tags: vec!["tag3".to_owned(), "tag4".to_owned()],
    };

    let client = reqwest::Client::new();
    let res = client
        .put(&format!(
            "{}/gestures/ce27c124-e47b-490f-b8fe-3f37d5dbbef6",
            address
        ))
        .json(&updatable_gesture)
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
            tags: vec!["tag3".to_owned(), "tag4".to_owned()],
            descriptions: vec![],
            meanings: vec![],
            pictures: vec![]
        }],
        gestures
    )
}
