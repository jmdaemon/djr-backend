use actix_cors::Cors;
use actix_web::{get, http::header, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize};

// use djr_backend::{api::{self, Database}, backend, models};
use djr_backend::{api, backend, models};
use simple_logger::SimpleLogger;

// 1. Create database facade
// 2. Create customer API using database facade
// 3. Implement customer database backend (stub + diesel backend)

// TODO:
// - Mixed Mock API requires an entirely different set of entities
// across mock + diesel backends. May be smarter to drop one of the backends
// and stub the data using a separate database connection.
// - API works but doesn't provide mocked data yet for use in front-end design.
// This should be solved when we finish the complete migration to the diesel backend,
// and create some basic test data for use in our rest API
//
// Overall, we should rethink our mocking strategy or follow through with the rest
// of the diesel backend integration

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

/// Send heartbeat response checks for testing
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

    // let djr_db: Database = Box::new(backend::MockBackend::new());
    // let djr_db: Database = backend::MySQLBackend::new();
    let djr_db = backend::MySQLBackend::new();
    let app_data = web::Data::new(djr_db);

    HttpServer::new(move ||
        App::new() 
            // Enable CORS
            .wrap(
                Cors::default()
                // add specific origin to allowed origin list
                .allowed_origin("http://localhost:3000")
                // allow any port on localhost
                .allowed_origin_fn(|origin, _req_head| {
                    origin.as_bytes().starts_with(b"http://localhost")
                    })
                    // set allowed methods list
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    // set allowed request header list
                    .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
                    // add header to allowed list
                    .allowed_header(header::CONTENT_TYPE)
                    // set list of headers that are safe to expose
                    .expose_headers(&[header::CONTENT_DISPOSITION])
                    // allow cURL/HTTPie from working without providing Origin headers
                    .block_on_origin_mismatch(false)
                    // set preflight cache TTL
                    .max_age(3600),
            )
            // Setup Routes
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
