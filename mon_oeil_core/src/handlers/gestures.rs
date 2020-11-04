use actix_web::{web, web::Json, HttpRequest, HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{db, models::*, ApiError, Conf};
use mon_oeil_auth_shared::valid_jwt_admin;

pub async fn get_gestures(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
) -> Result<impl Responder, ApiError> {
    let gestures = db.get().await.map_err(ApiError::from)?;
    let gestures = gestures.gestures().await.map_err(ApiError::from)?;

    Ok(Json(gestures))
}

pub async fn post_gesture(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    gesture: web::Json<NewGesture>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials).map_err(ApiError::from)?;

    let gesture = gesture.into_inner();

    let client = db.get().await.map_err(ApiError::from)?;
    client.add_gesture(gesture).await.map_err(ApiError::from)?;

    Ok(HttpResponse::Created().finish())
}

pub async fn delete_gesture(
    _req: HttpRequest,
    db: web::Data<db::DbPool>,
    id: web::Path<String>,
    conf: web::Data<Conf>,
    credentials: BearerAuth,
) -> Result<HttpResponse, ApiError> {
    valid_jwt_admin(&conf.hs256_private_key, &credentials).map_err(ApiError::from)?;

    let client = db.get().await.map_err(ApiError::from)?;
    client
        .delete_gesture_cascade(&id)
        .await
        .map_err(ApiError::from)?;

    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{middleware::Logger, test, web, App};
    use bytes::Bytes;

    const ADMIN_TOKEN: &str = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjIyMDM0MDcxOTgsImxldmVsIjoiQWRtaW4ifQ.RLE2du-ICZ0mlFl02YytZC02Xk0U5qyNRBxhi_-SvY8";

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

        let req = test::TestRequest::with_header("content-type", "application/json").to_request();
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
}
