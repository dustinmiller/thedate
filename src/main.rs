use actix_web::{middleware::Logger, web, App, HttpServer};
use thedate::{config::Config, health_check, home};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::from_env();
    let bind_addr = config.bind_address();

    log::info!("Starting thedate server on {}", bind_addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(home))
            .route("/health", web::get().to(health_check))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
