use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

use mon_oeil_srv::{auth, core, cors};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let hs256_private_key = std::env::var("HS256_PRIVATE_KEY").unwrap();

    let db_pool = mon_oeil_db::connect_db();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(cors())
            .configure(|mut config| {
                auth::app_config(&mut config, &db_pool, &hs256_private_key);
                core::app_config(&mut config, &db_pool, &hs256_private_key);
            })
    })
    .bind(format!(
        "0.0.0.0:{}",
        std::env::var("PORT").expect("Need env var PORT")
    ))?
    .run()
    .await
}
