use actix_web::{web, App, HttpServer};
use thedate::{config::Config, health_check, home};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let bind_addr = config.bind_address();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .route("/health", web::get().to(health_check))
    })
    .bind(bind_addr)?
    .run()
    .await
}
