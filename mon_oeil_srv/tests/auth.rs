#[macro_use]
extern crate serial_test;
use actix_web::http::StatusCode;

mod utils;

use mon_oeil_auth::*;
use mon_oeil_auth_shared::*;
use utils::setup;

#[actix_rt::test]
#[serial]
async fn login_as_admin() {
    setup::reset_db();
    setup::insert_user();

    let address = setup::spawn_app();

    let credential = Credentials {
        username: "user_test".to_owned(),
        password: "password_test".to_owned(),
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!("{}/login", address))
        .json(&credential)
        .send()
        .await
        .unwrap();

    assert!(res.status().is_success());
    let jwt = res.text().await.unwrap();
    let jwt = jwt.replace("\"", "");

    let payload = decode_jwt(&setup::CONF.hs256_private_key, &jwt).unwrap();
    assert_eq!(Level::Admin, payload.level);
}

#[actix_rt::test]
#[serial]
async fn login_with_wrong_password_is_unauthorized() {
    setup::reset_db();
    setup::insert_user();

    let address = setup::spawn_app();

    let credential = Credentials {
        username: "user_test".to_owned(),
        password: "wrong_password".to_owned(),
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!("{}/login", address))
        .json(&credential)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn login_with_wrong_username_is_unauthorized() {
    setup::reset_db();
    setup::insert_user();

    let address = setup::spawn_app();

    let credential = Credentials {
        username: "user_wrong".to_owned(),
        password: "password_test".to_owned(),
    };

    let client = reqwest::Client::new();
    let res = client
        .post(&format!("{}/login", address))
        .json(&credential)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}
