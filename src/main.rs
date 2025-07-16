use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize};

use djr_backend::{api::{self, Database}, backend, models};
use simple_logger::SimpleLogger;

// 1. Create database facade
// 2. Create customer API using database facade
// 3. Implement customer database backend (stub + diesel backend)

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}


async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Enable logging
    SimpleLogger::new().init().unwrap();

    // Set RUST_LOG from within main
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
    }

    let djr_db: Database = Box::new(backend::MockBackend::new());
    let app_data = web::Data::new(djr_db);

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .configure(api::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
