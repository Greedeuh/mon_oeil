use dotenv;
use mon_oeil_db::*;
use std::env::var;
use tokio_postgres::NoTls;
use tokio_test::block_on;

#[test]
fn connect_should_succeed() {
    let (host, port, dbname, user, password) = db_conf();
    let client = GestureClientPool::connect(&host, &port, &user, &password, &dbname);
    client.unwrap();
}

#[test]
fn connect_and_get_should_succeed() {
    let (host, port, dbname, user, password) = db_conf();
    let client = GestureClientPool::connect(&host, &port, &user, &password, &dbname);
    client.unwrap();
}

#[cfg(test)]
mod gestures {

    use super::*;

    #[test]
    fn user() {
        reset_db();
        insert_user();
        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let user = block_on(async {
            let client = pool.get().await?;
            client.get_user("user_test").await
        });

        assert_eq!(
            Ok(Some(User {
                username: "user_test".to_owned(),
                password: "password_test".to_owned()
            })),
            user
        );
    }

    #[test]
    fn no_gesture_in_db_should_return_nothing() {
        reset_db();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;
            client.gestures().await
        })
        .unwrap();

        assert_eq!(Vec::<Gesture>::new(), gestures);
    }

    #[test]
    fn gesture_without_links_in_db_should_return_them() {
        insert_gesture_without_links();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;

            client.gestures().await
        })
        .unwrap();

        assert_eq!(
            vec![Gesture {
                id: "ce27c124-e47b-490f-b8fe-3f37d5dbbef6".to_owned(),
                tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                descriptions: vec![],
                meanings: vec![],
                pictures: vec![]
            }],
            gestures
        );
    }

    #[test]
    fn _2_gestures_with_full_links_in_db_should_return_them() {
        insert_2_gestures_with_full_links();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;

            client.gestures().await
        })
        .unwrap();

        assert_eq!(
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
                                    langs: vec!["fr".to_owned(), "us".to_owned()]
                                },
                                Meaning {
                                    id: "45dca590-6bc4-4e4b-ad0c-0fe57a3a9643".to_owned(),
                                    value: "Un petit meaning".to_owned(),
                                    langs: vec!["fr".to_owned(), "us".to_owned()]
                                }
                            ]
                        },
                        Description {
                            id: "1c53f9ad-98b4-444c-9ec9-e8f92f1e5d28".to_owned(),
                            value: "Une petite description".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                            meanings: vec![]
                        }
                    ],
                    meanings: vec![
                        Meaning {
                            id: "59c25147-021e-4584-9c35-97cbf060cc89".to_owned(),
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        },
                        Meaning {
                            id: "02ca8fb9-c56e-4e45-b13e-98a6732f780a".to_owned(),
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        }
                    ],
                    pictures: vec![
                        Picture {
                            id: "283e7b04-7c13-4154-aafe-8e55b6960fe3".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        },
                        Picture {
                            id: "03b9bfc6-fa22-4ffb-9464-93c1be842ace".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        }
                    ]
                },
                Gesture {
                    id: "16991982-1752-4aa0-bb22-db3fbceb3780".to_owned(),
                    tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                    descriptions: vec![Description {
                        id: "cdbcd8fb-3d6d-4f09-86ba-37a6ec1dd293".to_owned(),
                        value: "Une petite description".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                        meanings: vec![]
                    }],
                    meanings: vec![Meaning {
                        id: "4719b1d7-2810-4f7d-865d-03ee44cf0add".to_owned(),
                        value: "Un petit meaning".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()]
                    }],
                    pictures: vec![Picture {
                        id: "6e1ee88d-fd97-488c-9aa8-6b66a3f3e714".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()]
                    }]
                }
            ],
            gestures
        );
    }
}

#[cfg(test)]
mod delete_gesture {

    use super::*;

    #[test]
    fn nothing_in_db_should_throw() {
        reset_db();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let res = block_on(async {
            let client = pool.get().await?;
            client
                .delete_gesture_cascade("ce27c124-e47b-490f-b8fe-3f37d5dbbef6")
                .await
        });

        match res {
            Err(_) => (),
            _ => panic!("Should have Err(DbError::NotFound)"),
        }
    }

    #[test]
    fn _1_of_2_should_return_1() {
        insert_2_gestures_with_full_links();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;
            client
                .delete_gesture_cascade("ce27c124-e47b-490f-b8fe-3f37d5dbbef6")
                .await?;

            client.gestures().await
        })
        .unwrap();
        assert_eq!(
            vec![Gesture {
                id: "16991982-1752-4aa0-bb22-db3fbceb3780".to_owned(),
                tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                descriptions: vec![Description {
                    id: "cdbcd8fb-3d6d-4f09-86ba-37a6ec1dd293".to_owned(),
                    value: "Une petite description".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                    meanings: vec![]
                }],
                meanings: vec![Meaning {
                    id: "4719b1d7-2810-4f7d-865d-03ee44cf0add".to_owned(),
                    value: "Un petit meaning".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()]
                }],
                pictures: vec![Picture {
                    id: "6e1ee88d-fd97-488c-9aa8-6b66a3f3e714".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()]
                }]
            }],
            gestures
        );
    }
}

#[cfg(test)]
mod delete_decription {

    use super::*;

    #[test]
    fn nothing_in_db_should_throw() {
        reset_db();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let res = block_on(async {
            let client = pool.get().await?;
            client
                .delete_description_cascade("ce27c124-e47b-490f-b8fe-3f37d5dbbef6")
                .await
        });

        match res {
            Err(_) => (),
            _ => panic!("Should have Err(DbError::NotFound)"),
        }
    }

    #[test]
    fn _1_of_1gesture_should_return_gestures_updated() {
        insert_2_gestures_with_full_links();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;
            client
                .delete_description_cascade("2ae70884-97bd-401d-8f43-d1778d4502d2")
                .await?;

            client.gestures().await
        })
        .unwrap();
        assert_eq!(
            vec![
                Gesture {
                    id: "ce27c124-e47b-490f-b8fe-3f37d5dbbef6".to_owned(),
                    tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                    descriptions: vec![Description {
                        id: "1c53f9ad-98b4-444c-9ec9-e8f92f1e5d28".to_owned(),
                        value: "Une petite description".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                        meanings: vec![]
                    }],
                    meanings: vec![
                        Meaning {
                            id: "59c25147-021e-4584-9c35-97cbf060cc89".to_owned(),
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        },
                        Meaning {
                            id: "02ca8fb9-c56e-4e45-b13e-98a6732f780a".to_owned(),
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        }
                    ],
                    pictures: vec![
                        Picture {
                            id: "283e7b04-7c13-4154-aafe-8e55b6960fe3".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        },
                        Picture {
                            id: "03b9bfc6-fa22-4ffb-9464-93c1be842ace".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        }
                    ]
                },
                Gesture {
                    id: "16991982-1752-4aa0-bb22-db3fbceb3780".to_owned(),
                    tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                    descriptions: vec![Description {
                        id: "cdbcd8fb-3d6d-4f09-86ba-37a6ec1dd293".to_owned(),
                        value: "Une petite description".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                        meanings: vec![]
                    }],
                    meanings: vec![Meaning {
                        id: "4719b1d7-2810-4f7d-865d-03ee44cf0add".to_owned(),
                        value: "Un petit meaning".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()]
                    }],
                    pictures: vec![Picture {
                        id: "6e1ee88d-fd97-488c-9aa8-6b66a3f3e714".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()]
                    }]
                }
            ],
            gestures
        );
    }
}

#[cfg(test)]
mod delete_meanings {

    use super::*;

    #[test]
    fn nothing_in_db_should_throw() {
        reset_db();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let res = block_on(async {
            let client = pool.get().await?;
            client
                .delete_meaning("e2c6eee0-49a7-49c4-9a0f-a9c6e6f668d8")
                .await
        });

        match res {
            Err(_) => (),
            _ => panic!("Should have Err(DbError::NotFound)"),
        }
    }

    #[test]
    fn _1_of_1gesture_should_return_gestures_updated() {
        insert_2_gestures_with_full_links();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;
            client
                .delete_meaning("59c25147-021e-4584-9c35-97cbf060cc89")
                .await?;

            client.gestures().await
        })
        .unwrap();
        assert_eq!(
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
                                    langs: vec!["fr".to_owned(), "us".to_owned()]
                                },
                                Meaning {
                                    id: "45dca590-6bc4-4e4b-ad0c-0fe57a3a9643".to_owned(),
                                    value: "Un petit meaning".to_owned(),
                                    langs: vec!["fr".to_owned(), "us".to_owned()]
                                }
                            ]
                        },
                        Description {
                            id: "1c53f9ad-98b4-444c-9ec9-e8f92f1e5d28".to_owned(),
                            value: "Une petite description".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                            meanings: vec![]
                        }
                    ],
                    meanings: vec![Meaning {
                        id: "02ca8fb9-c56e-4e45-b13e-98a6732f780a".to_owned(),
                        value: "Un petit meaning".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()]
                    }],
                    pictures: vec![
                        Picture {
                            id: "283e7b04-7c13-4154-aafe-8e55b6960fe3".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        },
                        Picture {
                            id: "03b9bfc6-fa22-4ffb-9464-93c1be842ace".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        }
                    ]
                },
                Gesture {
                    id: "16991982-1752-4aa0-bb22-db3fbceb3780".to_owned(),
                    tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                    descriptions: vec![Description {
                        id: "cdbcd8fb-3d6d-4f09-86ba-37a6ec1dd293".to_owned(),
                        value: "Une petite description".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                        meanings: vec![]
                    }],
                    meanings: vec![Meaning {
                        id: "4719b1d7-2810-4f7d-865d-03ee44cf0add".to_owned(),
                        value: "Un petit meaning".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()]
                    }],
                    pictures: vec![Picture {
                        id: "6e1ee88d-fd97-488c-9aa8-6b66a3f3e714".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()]
                    }]
                }
            ],
            gestures
        );
    }

    #[test]
    fn _1_of_1description_should_return_gestures_updated() {
        insert_2_gestures_with_full_links();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;
            client
                .delete_meaning("e2c6eee0-49a7-49c4-9a0f-a9c6e6f668d8")
                .await?;

            client.gestures().await
        })
        .unwrap();
        assert_eq!(
            vec![
                Gesture {
                    id: "ce27c124-e47b-490f-b8fe-3f37d5dbbef6".to_owned(),
                    tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                    descriptions: vec![
                        Description {
                            id: "2ae70884-97bd-401d-8f43-d1778d4502d2".to_owned(),
                            value: "Une petite description".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                            meanings: vec![Meaning {
                                id: "45dca590-6bc4-4e4b-ad0c-0fe57a3a9643".to_owned(),
                                value: "Un petit meaning".to_owned(),
                                langs: vec!["fr".to_owned(), "us".to_owned()]
                            }]
                        },
                        Description {
                            id: "1c53f9ad-98b4-444c-9ec9-e8f92f1e5d28".to_owned(),
                            value: "Une petite description".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                            meanings: vec![]
                        }
                    ],
                    meanings: vec![
                        Meaning {
                            id: "59c25147-021e-4584-9c35-97cbf060cc89".to_owned(),
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        },
                        Meaning {
                            id: "02ca8fb9-c56e-4e45-b13e-98a6732f780a".to_owned(),
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        }
                    ],
                    pictures: vec![
                        Picture {
                            id: "283e7b04-7c13-4154-aafe-8e55b6960fe3".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        },
                        Picture {
                            id: "03b9bfc6-fa22-4ffb-9464-93c1be842ace".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        }
                    ]
                },
                Gesture {
                    id: "16991982-1752-4aa0-bb22-db3fbceb3780".to_owned(),
                    tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                    descriptions: vec![Description {
                        id: "cdbcd8fb-3d6d-4f09-86ba-37a6ec1dd293".to_owned(),
                        value: "Une petite description".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                        meanings: vec![]
                    }],
                    meanings: vec![Meaning {
                        id: "4719b1d7-2810-4f7d-865d-03ee44cf0add".to_owned(),
                        value: "Un petit meaning".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()]
                    }],
                    pictures: vec![Picture {
                        id: "6e1ee88d-fd97-488c-9aa8-6b66a3f3e714".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()]
                    }]
                }
            ],
            gestures
        );
    }
}

#[cfg(test)]
mod delete_picture {

    use super::*;

    #[test]
    fn nothing_in_db_should_throw() {
        reset_db();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let res = block_on(async {
            let client = pool.get().await?;
            client
                .delete_meaning("283e7b04-7c13-4154-aafe-8e55b6960fe3")
                .await
        });

        match res {
            Err(_) => (),
            _ => panic!("Should have Err(DbError::NotFound)"),
        }
    }

    #[test]
    fn _1_of_1description_should_return_gestures_updated() {
        insert_2_gestures_with_full_links();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;
            client
                .delete_picture("283e7b04-7c13-4154-aafe-8e55b6960fe3")
                .await?;

            client.gestures().await
        })
        .unwrap();
        assert_eq!(
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
                                    langs: vec!["fr".to_owned(), "us".to_owned()]
                                },
                                Meaning {
                                    id: "45dca590-6bc4-4e4b-ad0c-0fe57a3a9643".to_owned(),
                                    value: "Un petit meaning".to_owned(),
                                    langs: vec!["fr".to_owned(), "us".to_owned()]
                                }
                            ]
                        },
                        Description {
                            id: "1c53f9ad-98b4-444c-9ec9-e8f92f1e5d28".to_owned(),
                            value: "Une petite description".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                            meanings: vec![]
                        }
                    ],
                    meanings: vec![
                        Meaning {
                            id: "59c25147-021e-4584-9c35-97cbf060cc89".to_owned(),
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        },
                        Meaning {
                            id: "02ca8fb9-c56e-4e45-b13e-98a6732f780a".to_owned(),
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()]
                        }
                    ],
                    pictures: vec![Picture {
                        id: "03b9bfc6-fa22-4ffb-9464-93c1be842ace".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()]
                    }]
                },
                Gesture {
                    id: "16991982-1752-4aa0-bb22-db3fbceb3780".to_owned(),
                    tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                    descriptions: vec![Description {
                        id: "cdbcd8fb-3d6d-4f09-86ba-37a6ec1dd293".to_owned(),
                        value: "Une petite description".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                        meanings: vec![]
                    }],
                    meanings: vec![Meaning {
                        id: "4719b1d7-2810-4f7d-865d-03ee44cf0add".to_owned(),
                        value: "Un petit meaning".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()]
                    }],
                    pictures: vec![Picture {
                        id: "6e1ee88d-fd97-488c-9aa8-6b66a3f3e714".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()]
                    }]
                }
            ],
            gestures
        );
    }
}

#[cfg(test)]
mod add {

    use super::*;

    #[test]
    fn gesture() {
        reset_db();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;
            client
                .add_gesture(NewGesture {
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
                .await?;

            client.gestures().await
        })
        .unwrap();

        let gres = &gestures[0];

        let gex = Gesture {
            id: gres.id.clone(),
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
            descriptions: vec![
                Description {
                    id: gres.descriptions[0].id.clone(),
                    value: "Une petite description".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                    meanings: vec![
                        Meaning {
                            id: gres.descriptions[0].meanings[0].id.clone(),
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                        },
                        Meaning {
                            id: gres.descriptions[0].meanings[1].id.clone(),
                            value: "Un petit meaning".to_owned(),
                            langs: vec!["fr".to_owned(), "us".to_owned()],
                        },
                    ],
                },
                Description {
                    id: gres.descriptions[1].id.clone(),
                    value: "Une petite description".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                    meanings: vec![],
                },
            ],
            meanings: vec![
                Meaning {
                    id: gres.meanings[0].id.clone(),
                    value: "Un petit meaning".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                },
                Meaning {
                    id: gres.meanings[1].id.clone(),
                    value: "Un petit meaning".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                },
            ],
            pictures: vec![
                Picture {
                    id: gres.pictures[0].id.clone(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                },
                Picture {
                    id: gres.pictures[1].id.clone(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                },
            ],
        };

        assert_eq!(vec![gex], gestures);
    }

    #[test]
    fn description() {
        insert_gesture_without_links();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;
            client
                .add_description(
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
                    "ce27c124-e47b-490f-b8fe-3f37d5dbbef6",
                )
                .await?;

            client.gestures().await
        })
        .unwrap();

        let gres = &gestures[0];

        let gex = Gesture {
            id: gres.id.clone(),
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
            descriptions: vec![Description {
                id: gres.descriptions[0].id.clone(),
                value: "Une petite description".to_owned(),
                langs: vec!["fr".to_owned(), "us".to_owned()],
                meanings: vec![
                    Meaning {
                        id: gres.descriptions[0].meanings[0].id.clone(),
                        value: "Un petit meaning".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                    },
                    Meaning {
                        id: gres.descriptions[0].meanings[1].id.clone(),
                        value: "Un petit meaning".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                    },
                ],
            }],
            meanings: vec![],
            pictures: vec![],
        };

        assert_eq!(vec![gex], gestures);
    }

    #[test]
    fn gesture_s_meaning() {
        insert_gesture_without_links();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;
            client
                .add_meaning(
                    NewMeaning {
                        value: "Un petit meaning".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                    },
                    Some("ce27c124-e47b-490f-b8fe-3f37d5dbbef6"),
                    None,
                )
                .await?;

            client.gestures().await
        })
        .unwrap();

        let gres = &gestures[0];

        let gex = Gesture {
            id: gres.id.clone(),
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
            descriptions: vec![],
            meanings: vec![Meaning {
                id: gres.meanings[0].id.clone(),
                value: "Un petit meaning".to_owned(),
                langs: vec!["fr".to_owned(), "us".to_owned()],
            }],
            pictures: vec![],
        };

        assert_eq!(vec![gex], gestures);
    }

    #[test]
    fn description_s_meaning() {
        insert_gesture_with_description();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;
            client
                .add_meaning(
                    NewMeaning {
                        value: "Un petit meaning".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                    },
                    None,
                    Some("2ae70884-97bd-401d-8f43-d1778d4502d2"),
                )
                .await?;

            client.gestures().await
        })
        .unwrap();

        let gres = &gestures[0];

        let gex = Gesture {
            id: gres.id.clone(),
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
            descriptions: vec![Description {
                id: gres.descriptions[0].id.clone(),
                value: "Une petite description".to_owned(),
                langs: vec!["fr".to_owned(), "us".to_owned()],
                meanings: vec![Meaning {
                    id: gres.descriptions[0].meanings[0].id.clone(),
                    value: "Un petit meaning".to_owned(),
                    langs: vec!["fr".to_owned(), "us".to_owned()],
                }],
            }],
            meanings: vec![],
            pictures: vec![],
        };

        assert_eq!(vec![gex], gestures);
    }

    #[test]
    fn picture() {
        insert_gesture_without_links();

        let (host, port, dbname, user, password) = db_conf();
        let pool = GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap();

        let gestures = block_on(async {
            let client = pool.get().await?;
            client
                .add_picture(
                    NewPicture {
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                    },
                    "ce27c124-e47b-490f-b8fe-3f37d5dbbef6",
                )
                .await?;

            client.gestures().await
        })
        .unwrap();

        let gres = &gestures[0];

        let gex = Gesture {
            id: gres.id.clone(),
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
            descriptions: vec![],
            meanings: vec![],
            pictures: vec![Picture {
                id: gres.pictures[0].id.clone(),
                langs: vec!["fr".to_owned(), "us".to_owned()],
            }],
        };

        assert_eq!(vec![gex], gestures);
    }
}

fn insert_gesture_without_links() {
    reset_db();

    block_on(async {
        let client = connect().await;
        client.execute(r#"INSERT INTO gestures(id_gesture, tags) VALUES ('ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"tag1", "tag2"}')"#, &[]).await.unwrap();
    });
}

fn insert_gesture_with_description() {
    reset_db();

    block_on(async {
        let client = connect().await;
        client.execute(r#"INSERT INTO gestures(id_gesture, tags) VALUES ('ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"tag1", "tag2"}')"#, &[]).await.unwrap();
        client.execute(r#"INSERT INTO descriptions(
            id_description, id_gesture, val, langs)
            VALUES ('2ae70884-97bd-401d-8f43-d1778d4502d2', 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', 'Une petite description', '{"fr", "us"}');"#, &[]).await.unwrap();
    });
}

fn insert_user() {
    reset_db();

    block_on(async {
        let client = connect().await;
        client
            .execute(
                r#"INSERT INTO users(username, password) VALUES ('user_test', 'password_test')"#,
                &[],
            )
            .await
            .unwrap();
    });
}

fn insert_2_gestures_with_full_links() {
    reset_db();

    block_on(async {
        let client = connect().await;
        client.execute(r#"INSERT INTO gestures(id_gesture, tags) VALUES ('ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"tag1", "tag2"}')"#, &[]).await.unwrap();
        client.execute(r#"INSERT INTO gestures(id_gesture, tags) VALUES ('16991982-1752-4aa0-bb22-db3fbceb3780', '{"tag1", "tag2"}')"#, &[]).await.unwrap();

        client.execute(r#"INSERT INTO descriptions(
            id_description, id_gesture, val, langs)
            VALUES ('2ae70884-97bd-401d-8f43-d1778d4502d2', 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', 'Une petite description', '{"fr", "us"}');"#, &[]).await.unwrap();
        client.execute(r#"INSERT INTO descriptions(
            id_description, id_gesture, val, langs)
            VALUES ('1c53f9ad-98b4-444c-9ec9-e8f92f1e5d28', 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', 'Une petite description', '{"fr", "us"}');"#, &[]).await.unwrap();
        client.execute(r#"INSERT INTO descriptions(
            id_description, id_gesture, val, langs)
            VALUES ('cdbcd8fb-3d6d-4f09-86ba-37a6ec1dd293', '16991982-1752-4aa0-bb22-db3fbceb3780', 'Une petite description', '{"fr", "us"}');"#, &[]).await.unwrap();

        client.execute(r#"INSERT INTO meanings(
            id_meaning, id_description, id_gesture, val, langs)
            VALUES ('59c25147-021e-4584-9c35-97cbf060cc89', null, 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', 'Un petit meaning', '{"fr", "us"}');"#, &[]).await.unwrap();
        client.execute(r#"INSERT INTO meanings(
            id_meaning, id_description, id_gesture, val, langs)
            VALUES ('02ca8fb9-c56e-4e45-b13e-98a6732f780a', null, 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', 'Un petit meaning', '{"fr", "us"}');"#, &[]).await.unwrap();
        client.execute(r#"INSERT INTO meanings(
            id_meaning, id_description, id_gesture, val, langs)
            VALUES ('4719b1d7-2810-4f7d-865d-03ee44cf0add', null, '16991982-1752-4aa0-bb22-db3fbceb3780', 'Un petit meaning', '{"fr", "us"}');"#, &[]).await.unwrap();
        client.execute(r#"INSERT INTO meanings(
            id_meaning, id_description, id_gesture, val, langs)
            VALUES ('e2c6eee0-49a7-49c4-9a0f-a9c6e6f668d8', '2ae70884-97bd-401d-8f43-d1778d4502d2', null, 'Un petit meaning', '{"fr", "us"}');"#, &[]).await.unwrap();
        client.execute(r#"INSERT INTO meanings(
            id_meaning, id_description, id_gesture, val, langs)
            VALUES ('45dca590-6bc4-4e4b-ad0c-0fe57a3a9643', '2ae70884-97bd-401d-8f43-d1778d4502d2', null, 'Un petit meaning', '{"fr", "us"}');"#, &[]).await.unwrap();

        client.execute(r#"INSERT INTO pictures(
            id_picture, id_gesture, langs)
            VALUES ('283e7b04-7c13-4154-aafe-8e55b6960fe3', 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"fr", "us"}');"#, &[]).await.unwrap();
        client.execute(r#"INSERT INTO pictures(
            id_picture, id_gesture, langs)
            VALUES ('03b9bfc6-fa22-4ffb-9464-93c1be842ace', 'ce27c124-e47b-490f-b8fe-3f37d5dbbef6', '{"fr", "us"}');"#, &[]).await.unwrap();
        client.execute(r#"INSERT INTO pictures(
            id_picture, id_gesture, langs)
            VALUES ('6e1ee88d-fd97-488c-9aa8-6b66a3f3e714', '16991982-1752-4aa0-bb22-db3fbceb3780', '{"fr", "us"}');"#, &[]).await.unwrap();
    });
}

fn reset_db() {
    block_on(async {
        let client = connect().await;

        client.execute("DELETE FROM gestures", &[]).await.unwrap();
        client
            .execute("DELETE FROM descriptions", &[])
            .await
            .unwrap();
        client.execute("DELETE FROM meanings", &[]).await.unwrap();
        client.execute("DELETE FROM pictures", &[]).await.unwrap();
        client.execute("DELETE FROM users", &[]).await.unwrap();
    })
}

async fn connect() -> tokio_postgres::Client {
    let (host, port, dbname, user, password) = db_conf();

    let connection = tokio_postgres::connect(
        &format!(
            "host={} port={} user={} password={} dbname={}",
            host, port, user, password, dbname
        ),
        NoTls,
    )
    .await
    .unwrap();

    let (client, connection) = connection;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

fn db_conf() -> (String, String, String, String, String) {
    dotenv::dotenv().ok();
    (
        var("PG_HOST").unwrap(),
        var("PG_PORT").unwrap(),
        var("PG_DB_NAME").unwrap(),
        var("PG_USER").unwrap(),
        var("PG_PWD").unwrap(),
    )
}
