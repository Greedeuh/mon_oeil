use actix_cors::Cors;
use actix_web::{http, middleware::Logger, App, HttpServer};
use env_logger::Env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let hs256_private_key = std::env::var("HS256_PRIVATE_KEY").unwrap();

    let db_pool = mon_oeil_db::connect_db();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::new()
                    .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .configure(|mut config| {
                mon_oeil_auth::app_config(&mut config, &db_pool, &hs256_private_key);
                mon_oeil_core::app_config(&mut config, &db_pool, &hs256_private_key);
            })
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
