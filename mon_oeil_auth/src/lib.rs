use actix_web::web;

mod db;
mod handlers;
mod models;

use handlers::*;

pub struct Conf {
    pub hs256_private_key: String,
}

pub fn app_config(
    config: &mut web::ServiceConfig,
    db_pool: &mon_oeil_db::GestureClientPool,
    hs256_private_key: &str,
) {
    config
        .data(Conf {
            hs256_private_key: hs256_private_key.to_owned(),
        })
        .data(db::DbPool::new(db_pool.clone()))
        .route("/login", web::post().to(login));
}
