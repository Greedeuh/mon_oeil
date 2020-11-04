use actix_web::{error, http, web};
use failure::Fail;
use serde::{Deserialize, Serialize};

mod db;
mod handlers;
mod model;

use handlers::*;
use mon_oeil_auth_shared as auth;

pub fn app_config(
    config: &mut web::ServiceConfig,
    db_pool: &mon_oeil_db::GestureClientPool,
    hs256_private_key: &str,
) {
    config
        .data(db::DbPool::new(db_pool.clone()))
        .data(Conf {
            hs256_private_key: hs256_private_key.to_owned(),
        })
        .route("/gestures", web::get().to(get_gestures))
        .route("/gestures", web::post().to(post_gesture))
        .route("/gestures/{id}", web::delete().to(delete_gesture))
        .route(
            "/gestures/{id_gesutre}/descriptions",
            web::post().to(post_description),
        )
        .route("/description", web::delete().to(delete_description))
        .route(
            "/gestures/{id_gesutre}/meanings",
            web::post().to(post_gesture_s_meaning),
        )
        .route(
            "/descriptions/{id_description}/meanings",
            web::post().to(post_description_s_meaning),
        )
        .route("/meanings", web::delete().to(delete_meaning))
        .route(
            "/gestures/{id_gesutre}/pictures",
            web::post().to(post_picture_meta),
        )
        .route("/pictures", web::delete().to(delete_picture))
        .route("/*", web::method(http::Method::OPTIONS).to(delete_picture));
}

pub struct Conf {
    pub hs256_private_key: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Fail)]
#[fail(display = "my error")]
pub struct ApiError(pub String);

// Use default implementation for `error_response()` method
impl error::ResponseError for ApiError {}

impl From<db::DbError> for ApiError {
    fn from(err: db::DbError) -> ApiError {
        ApiError(format!("{:?}", err))
    }
}

impl From<auth::JwtValidationError> for ApiError {
    fn from(err: auth::JwtValidationError) -> ApiError {
        ApiError(format!("{:?}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use model::*;
    const ADMIN_TOKEN: &str = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjIyMDM0MDcxOTgsImxldmVsIjoiQWRtaW4ifQ.RLE2du-ICZ0mlFl02YytZC02Xk0U5qyNRBxhi_-SvY8";

    #[cfg(test)]
    mod api {
        use super::*;
        use actix_web::{middleware::Logger, test, web, App};
        use bytes::Bytes;

        #[actix_rt::test]
        async fn get_gestures() {
            let mut pool = db::DbPool::faux();
            let mut client = db::DbClient::faux();
            faux::when!(client.gestures).safe_then(|_| {
                Ok(vec![
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
                            },
                            Picture {
                                id: "03b9bfc6-fa22-4ffb-9464-93c1be842ace".to_owned(),
                                langs: vec!["fr".to_owned(), "us".to_owned()],
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
                        }],
                    },
                ])
            });
            faux::when!(pool.get).once().safe_then(move |_| Ok(client));

            let mut app = test::init_service(
                App::new()
                    .data(pool)
                    .data(Conf {
                        hs256_private_key: "private_key".to_owned(),
                    })
                    .wrap(Logger::default())
                    .route("/", web::get().to(super::get_gestures)),
            )
            .await;

            let req =
                test::TestRequest::with_header("content-type", "application/json").to_request();
            let resp = test::call_service(&mut app, req).await;

            assert!(resp.status().is_success());

            let res = test::read_body(resp).await;
            assert_eq!(
                res,
                Bytes::from(
                    r#"[{"id":"ce27c124-e47b-490f-b8fe-3f37d5dbbef6","tags":["tag1","tag2"],"descriptions":[{"id":"2ae70884-97bd-401d-8f43-d1778d4502d2","value":"Une petite description","langs":["fr","us"],"meanings":[{"id":"e2c6eee0-49a7-49c4-9a0f-a9c6e6f668d8","value":"Un petit meaning","langs":["fr","us"]},{"id":"45dca590-6bc4-4e4b-ad0c-0fe57a3a9643","value":"Un petit meaning","langs":["fr","us"]}]},{"id":"1c53f9ad-98b4-444c-9ec9-e8f92f1e5d28","value":"Une petite description","langs":["fr","us"],"meanings":[]}],"meanings":[{"id":"59c25147-021e-4584-9c35-97cbf060cc89","value":"Un petit meaning","langs":["fr","us"]},{"id":"02ca8fb9-c56e-4e45-b13e-98a6732f780a","value":"Un petit meaning","langs":["fr","us"]}],"pictures":[{"id":"283e7b04-7c13-4154-aafe-8e55b6960fe3","langs":["fr","us"]},{"id":"03b9bfc6-fa22-4ffb-9464-93c1be842ace","langs":["fr","us"]}]},{"id":"16991982-1752-4aa0-bb22-db3fbceb3780","tags":["tag1","tag2"],"descriptions":[{"id":"cdbcd8fb-3d6d-4f09-86ba-37a6ec1dd293","value":"Une petite description","langs":["fr","us"],"meanings":[]}],"meanings":[{"id":"4719b1d7-2810-4f7d-865d-03ee44cf0add","value":"Un petit meaning","langs":["fr","us"]}],"pictures":[{"id":"6e1ee88d-fd97-488c-9aa8-6b66a3f3e714","langs":["fr","us"]}]}]"#
                )
            )
        }

        #[actix_rt::test]
        async fn add_gesture() {
            let mut pool = db::DbPool::faux();
            let mut client = db::DbClient::faux();
            faux::when!(client.add_gesture).safe_then(|_| Ok(()));
            faux::when!(pool.get).once().safe_then(move |_| Ok(client));

            let mut app = test::init_service(
                App::new()
                    .data(pool)
                    .data(Conf {
                        hs256_private_key: "private_key".to_owned(),
                    })
                    .wrap(Logger::default())
                    .route("/", web::post().to(post_gesture)),
            )
            .await;

            let req = test::TestRequest::post()
                .header("content-type", "application/json")
                .header("Authorization", ADMIN_TOKEN)
                .set_json(&NewGesture {
                    tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                    descriptions: vec![
                        NewDescription {
                            value: "Une petite description".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                            meanings: vec![
                                NewMeaning {
                                    value: "Un petit meaning".to_owned(),
                                    langs: vec!["fr".to_owned(), "us".to_owned()],
                                },
                                NewMeaning {
                                    value: "Un petit meaning".to_owned(),
                                    langs: vec!["fr".to_owned(), "us".to_owned()],
                                },
                            ],
                        },
                        NewDescription {
                            value: "Une petite description".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                            meanings: vec![],
                        },
                    ],
                    meanings: vec![
                        NewMeaning {
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                        },
                        NewMeaning {
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                        },
                    ],
                    pictures: vec![
                        NewPicture {
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                        },
                        NewPicture {
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                        },
                    ],
                })
                .to_request();
            let resp = test::call_service(&mut app, req).await;

            assert!(resp.status().is_success());
        }

        #[actix_rt::test]
        async fn delete_gesture() {
            let mut pool = db::DbPool::faux();
            let mut client = db::DbClient::faux();
            unsafe {
                // TODO : Comprendre quel est la différene avec le test post du dessus
                faux::when!(client.delete_gesture_cascade).then(|_| Ok(()));
            }
            faux::when!(pool.get).once().safe_then(move |_| Ok(client));

            let mut app = test::init_service(
                App::new()
                    .data(pool)
                    .data(Conf {
                        hs256_private_key: "private_key".to_owned(),
                    })
                    .wrap(Logger::default())
                    .route("/{id}", web::delete().to(super::delete_gesture)),
            )
            .await;

            let req = test::TestRequest::delete()
                .uri("/une_id")
                .header("content-type", "application/json")
                .header("Authorization", ADMIN_TOKEN)
                .to_request();
            let resp = test::call_service(&mut app, req).await;
            dbg!(resp.status());
            assert!(resp.status().is_success());
        }

        #[actix_rt::test]
        async fn add_description() {
            let mut pool = db::DbPool::faux();
            let mut client = db::DbClient::faux();
            unsafe {
                faux::when!(client.add_description).then(|_| Ok(()));
            }
            faux::when!(pool.get).once().safe_then(move |_| Ok(client));

            let mut app = test::init_service(
                App::new()
                    .data(pool)
                    .data(Conf {
                        hs256_private_key: "private_key".to_owned(),
                    })
                    .wrap(Logger::default())
                    .route("/{id_gesture}", web::post().to(post_description)),
            )
            .await;

            let req = test::TestRequest::post()
                .uri("/une_id_gesture")
                .header("content-type", "application/json")
                .header("Authorization", ADMIN_TOKEN)
                .set_json(&NewDescription {
                    value: "Une petite description".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                    meanings: vec![
                        NewMeaning {
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                        },
                        NewMeaning {
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                        },
                    ],
                })
                .to_request();
            let resp = test::call_service(&mut app, req).await;

            assert!(resp.status().is_success());
        }

        #[actix_rt::test]
        async fn delete_description() {
            let mut pool = db::DbPool::faux();
            let mut client = db::DbClient::faux();
            unsafe {
                // TODO : Comprendre quel est la différene avec le test post du dessus
                faux::when!(client.delete_description_cascade).then(|_| Ok(()));
            }
            faux::when!(pool.get).once().safe_then(move |_| Ok(client));

            let mut app = test::init_service(
                App::new()
                    .data(pool)
                    .data(Conf {
                        hs256_private_key: "private_key".to_owned(),
                    })
                    .wrap(Logger::default())
                    .route("/{id}", web::delete().to(super::delete_description)),
            )
            .await;

            let req = test::TestRequest::delete()
                .uri("/une_id")
                .header("content-type", "application/json")
                .header("Authorization", ADMIN_TOKEN)
                .to_request();
            let resp = test::call_service(&mut app, req).await;
            dbg!(resp.status());
            assert!(resp.status().is_success());
        }

        #[actix_rt::test]
        async fn add_gesture_s_meaning() {
            let mut pool = db::DbPool::faux();
            let mut client = db::DbClient::faux();
            unsafe {
                faux::when!(client.add_meaning).then(|_| Ok(()));
            }
            faux::when!(pool.get).once().safe_then(move |_| Ok(client));

            let mut app = test::init_service(
                App::new()
                    .data(pool)
                    .data(Conf {
                        hs256_private_key: "private_key".to_owned(),
                    })
                    .wrap(Logger::default())
                    .route("/{id_gesture}", web::post().to(post_gesture_s_meaning)),
            )
            .await;

            let req = test::TestRequest::post()
                .uri("/une_id_gesture")
                .header("content-type", "application/json")
                .header("Authorization", ADMIN_TOKEN)
                .set_json(&NewMeaning {
                    value: "Un petit meaning".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                })
                .to_request();
            let resp = test::call_service(&mut app, req).await;

            assert!(resp.status().is_success());
        }

        #[actix_rt::test]
        async fn add_decription_s_meaning() {
            let mut pool = db::DbPool::faux();
            let mut client = db::DbClient::faux();
            unsafe {
                faux::when!(client.add_meaning).then(|_| Ok(()));
            }
            faux::when!(pool.get).once().safe_then(move |_| Ok(client));

            let mut app = test::init_service(
                App::new()
                    .data(pool)
                    .data(Conf {
                        hs256_private_key: "private_key".to_owned(),
                    })
                    .wrap(Logger::default())
                    .route("/{id_gesture}", web::post().to(post_description_s_meaning)),
            )
            .await;

            let req = test::TestRequest::post()
                .uri("/une_id_gesture")
                .header("content-type", "application/json")
                .header("Authorization", ADMIN_TOKEN)
                .set_json(&NewMeaning {
                    value: "Un petit meaning".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                })
                .to_request();
            let resp = test::call_service(&mut app, req).await;

            assert!(resp.status().is_success());
        }

        #[actix_rt::test]
        async fn delete_meaning() {
            let mut pool = db::DbPool::faux();
            let mut client = db::DbClient::faux();
            unsafe {
                // TODO : Comprendre quel est la différene avec le test post du dessus
                faux::when!(client.delete_meaning).then(|_| Ok(()));
            }
            faux::when!(pool.get).once().safe_then(move |_| Ok(client));

            let mut app = test::init_service(
                App::new()
                    .data(pool)
                    .data(Conf {
                        hs256_private_key: "private_key".to_owned(),
                    })
                    .wrap(Logger::default())
                    .route("/{id}", web::delete().to(super::delete_meaning)),
            )
            .await;

            let req = test::TestRequest::delete()
                .uri("/une_id")
                .header("content-type", "application/json")
                .header("Authorization", ADMIN_TOKEN)
                .to_request();
            let resp = test::call_service(&mut app, req).await;
            dbg!(resp.status());
            assert!(resp.status().is_success());
        }

        #[actix_rt::test]
        async fn add_picture() {
            let mut pool = db::DbPool::faux();
            let mut client = db::DbClient::faux();
            unsafe {
                faux::when!(client.add_picture).then(|_| Ok(()));
            }
            faux::when!(pool.get).once().safe_then(move |_| Ok(client));

            let mut app = test::init_service(
                App::new()
                    .data(pool)
                    .data(Conf {
                        hs256_private_key: "private_key".to_owned(),
                    })
                    .wrap(Logger::default())
                    .route("/{id_gesture}", web::post().to(post_picture_meta)),
            )
            .await;

            let req = test::TestRequest::post()
                .uri("/une_id_gesture")
                .header("content-type", "application/json")
                .header("Authorization", ADMIN_TOKEN)
                .set_json(&NewDescription {
                    value: "Une petite description".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                    meanings: vec![
                        NewMeaning {
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                        },
                        NewMeaning {
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                        },
                    ],
                })
                .to_request();
            let resp = test::call_service(&mut app, req).await;

            assert!(resp.status().is_success());
        }

        #[actix_rt::test]
        async fn delete_picture() {
            let mut pool = db::DbPool::faux();
            let mut client = db::DbClient::faux();
            unsafe {
                // TODO : Comprendre quel est la différene avec le test post du dessus
                faux::when!(client.delete_picture).then(|_| Ok(()));
            }
            faux::when!(pool.get).once().safe_then(move |_| Ok(client));

            let mut app = test::init_service(
                App::new()
                    .data(pool)
                    .data(Conf {
                        hs256_private_key: "private_key".to_owned(),
                    })
                    .wrap(Logger::default())
                    .route("/{id}", web::delete().to(super::delete_picture)),
            )
            .await;

            let req = test::TestRequest::delete()
                .uri("/une_id")
                .header("content-type", "application/json")
                .header("Authorization", ADMIN_TOKEN)
                .to_request();
            let resp = test::call_service(&mut app, req).await;
            dbg!(resp.status());
            assert!(resp.status().is_success());
        }
    }
}
