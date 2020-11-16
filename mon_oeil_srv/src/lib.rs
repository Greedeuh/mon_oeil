use actix_cors::Cors;
use actix_web::{dev::Server, http, middleware::Logger, App, HttpServer};
use std::net::TcpListener;

pub mod auth;
pub mod core;
use mon_oeil_storage::*;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    run_with_storage(listener, build_storage)
}

fn build_storage() -> Storage {
    let bucket_name =
        std::env::var("GOOGLE_CLOUD_BUCKET").expect("GOOGLE_CLOUD_BUCKET as to be not set");

    Storage::new(&bucket_name)
}

pub fn run_with_storage(
    listener: TcpListener,
    build_storage: fn() -> Storage,
) -> Result<Server, std::io::Error> {
    let hs256_private_key = std::env::var("HS256_PRIVATE_KEY").unwrap();

    let db_pool = mon_oeil_db::connect_db();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(cors())
            .data(build_storage())
            .data(db_pool.clone())
            .data(build_storage())
            .configure(|mut config| {
                auth::app_config(&mut config, &hs256_private_key);
                core::app_config(&mut config, &hs256_private_key);
            })
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub fn cors() -> Cors {
    Cors::permissive()
}

pub struct Conf {
    pub hs256_private_key: String,
}

struct ApiError<T>(T);
