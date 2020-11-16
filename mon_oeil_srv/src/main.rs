use mon_oeil_srv::run;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        std::env::var("PORT").expect("Need env var PORT")
    ))
    .expect("Failed to bind random port");

    run(listener)?.await
}
