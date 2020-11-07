#[macro_use]
extern crate serial_test;
use actix_web::{http::StatusCode, middleware::Logger, test, App};

mod utils;

use mon_oeil_auth::*;
use mon_oeil_auth_shared::*;
use mon_oeil_srv::{auth, core, cors};
use utils::setup;

#[actix_rt::test]
#[serial]
async fn login_as_admin() {
    setup::reset_db();
    setup::insert_user();

    let mut app = test::init_service(App::new().wrap(Logger::default()).wrap(cors()).configure(
        |mut config| {
            auth::app_config(
                &mut config,
                &setup::CONF.db_pool,
                &setup::CONF.hs256_private_key,
            );
            core::app_config(
                &mut config,
                &setup::CONF.db_pool,
                &setup::CONF.hs256_private_key,
            );
        },
    ))
    .await;
    let req = test::TestRequest::post()
        .uri("/login")
        .header("content-type", "application/json")
        .set_json(&Credentials {
            username: "user_test".to_owned(),
            password: "password_test".to_owned(),
        })
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
    let jwt = test::read_body(resp).await;
    let jwt = std::str::from_utf8(&jwt).unwrap().to_owned();
    let jwt = jwt.replace("\"", "");

    let payload = decode_jwt(&setup::CONF.hs256_private_key, &jwt).unwrap();
    assert_eq!(Level::Admin, payload.level);
}

#[actix_rt::test]
#[serial]
async fn login_with_wrong_password_is_unauthorized() {
    setup::reset_db();
    setup::insert_user();

    let mut app = test::init_service(App::new().wrap(Logger::default()).wrap(cors()).configure(
        |mut config| {
            auth::app_config(
                &mut config,
                &setup::CONF.db_pool,
                &setup::CONF.hs256_private_key,
            );
            core::app_config(
                &mut config,
                &setup::CONF.db_pool,
                &setup::CONF.hs256_private_key,
            );
        },
    ))
    .await;
    let req = test::TestRequest::post()
        .uri("/login")
        .header("content-type", "application/json")
        .set_json(&Credentials {
            username: "user_test".to_owned(),
            password: "password_wrong".to_owned(),
        })
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[actix_rt::test]
#[serial]
async fn login_with_wrong_username_is_unauthorized() {
    setup::reset_db();
    setup::insert_user();

    let mut app = test::init_service(App::new().wrap(Logger::default()).wrap(cors()).configure(
        |mut config| {
            auth::app_config(
                &mut config,
                &setup::CONF.db_pool,
                &setup::CONF.hs256_private_key,
            );
            core::app_config(
                &mut config,
                &setup::CONF.db_pool,
                &setup::CONF.hs256_private_key,
            );
        },
    ))
    .await;
    let req = test::TestRequest::post()
        .uri("/login")
        .header("content-type", "application/json")
        .set_json(&Credentials {
            username: "user_wrong".to_owned(),
            password: "password_test".to_owned(),
        })
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}
