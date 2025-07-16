//! Define the REST API for CRUD operations on our MySQL db

use std::fmt::Error;

use actix_web::{delete, get, post, put, web};
use actix_web::{web::{
    Data,
    Json,
}, HttpResponse};
use serde::Serialize;
use crate::models::todo::Customer;

// API Routes

///! Backend API for use in our database providers/implementors
///! Defines the CRUD operations for our API
pub trait API {
    fn get_customers(&self) -> Vec<Customer>;
    fn create_customer(&self, customer: Customer) -> Result<Customer, Error>;
}

// A database is anything that implements our backend
// Must be Send + Sync because it will be shared between threads on the backend server
pub type Database = Box<dyn API + Send + Sync>;

// Endpoint: Customer
#[post("/customers")]
pub async fn create_customer(db: Data<Database>, new_customer: Json<Customer>) -> HttpResponse {
    let customer = db.create_customer(new_customer.into_inner());
    match customer {
        Ok(customer) => HttpResponse::Ok().json(customer),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/customers")]
// pub async fn get_customers(db: web::Data<Database>) -> HttpResponse {
pub async fn get_customers(db: web::Data<Database>) -> HttpResponse {
    let todos = db.get_customers();
    HttpResponse::Ok().json(todos)
}

#[get("/customers/{id}")]
pub async fn get_customer_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    // TODO: Implement
    healthcheck()

    // let customer = db.get_customer_by_id(&id);
    // match customer {
    //     Some(todo) => HttpResponse::Ok().json(todo),
    //     None => HttpResponse::NotFound().body("Customer not found"),
    // }
}

#[put("/customers/{id}")]
pub async fn update_customer_by_id(db: web::Data<Database>, id: web::Path<String>, updated_customer: web::Json<Customer>) -> HttpResponse {
    // TODO: Implement
    healthcheck()

    // let customer = db.update_customer_by_id(&id, updated_customer.into_inner());
    // match customer {
    //     Some(todo) => HttpResponse::Ok().json(customer),
    //     None => HttpResponse::NotFound().body("Todo not found"),
    // }
}

#[delete("/customers/{id}")]
pub async fn delete_customer_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    // TODO: Implement
    healthcheck()

    // let customer = db.delete_customer_by_id(&id);
    // match customer {
    //     Some(customer) => HttpResponse::Ok().json(customer),
    //     None => HttpResponse::NotFound().body("Todo not found"),
    // }
}

// Helper Functions
#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

fn healthcheck() -> HttpResponse {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

// Add all the routes to our REST api service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
            .service(create_customer)
            .service(get_customers)
            .service(get_customer_by_id)
            .service(update_customer_by_id)
            .service(delete_customer_by_id)

    );
}
