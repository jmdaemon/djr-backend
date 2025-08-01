//! Define the REST API for CRUD operations on our MySQL db

use std::fmt::Error;

use actix_web::{delete, get, post, put, web};
use actix_web::{web::{
    Data,
    Json,
}, HttpResponse};
use serde::Serialize;
use crate::backend::MySQLBackend;
// use crate::models::customer::mock::Customer;
use crate::models::customer::mysql::Customer;

// API Routes

/*
///! Backend API for use in our database providers/implementors
///! Defines the CRUD operations for our API
pub trait API {
    fn get_customers(&self) -> Vec<Customer>;
    // fn get_customer(&self, id: i32) -> Option<Customer>;

    fn create_customer(&self, customer: Customer) -> Result<Customer, Error>;
}
*/

// REST: API:
// Customer:
//  GET customer/{id}
//  GET customers
//  POST (JSON BODY) customers
// Employee:
//  GET employee/{id}
//  GET employees
//  POST (JSON BODY) employees
// Work Orders:
//  GET work-order/{id}


// A database is anything that implements our backend
// Must be Send + Sync because it will be shared between threads on the backend server
// pub type Database = Box<dyn API + Send + Sync>;

/// Retrieve all customers
#[get("/customers")]
pub async fn get_customers(db: web::Data<MySQLBackend>) -> HttpResponse {
    HttpResponse::Ok().json(db.get_customers())
}

/// Get customer by id
#[get("/customers/{id}")]
pub async fn get_customer_by_id(db: web::Data<MySQLBackend>, id: web::Path<i32>) -> HttpResponse {

    let customer = db.get_customer_by_id(*id);
    match customer {
        Some(customer) => HttpResponse::Ok().json(customer),
        None => HttpResponse::NotFound().body("Customer not found"),
    }
}

/// Create a new customer
#[post("/customers")]
pub async fn create_customer(db: Data<MySQLBackend>, new_customer: Json<Customer>) -> HttpResponse {
    let customer = db.create_customer(new_customer.into_inner());
    match customer {
        Ok(customer) => HttpResponse::Ok().json(customer),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Update customer by id
#[put("/customers/{id}")]
pub async fn update_customer_by_id(db: web::Data<MySQLBackend>, id: web::Path<i32>, updated_customer: web::Json<Customer>) -> HttpResponse {
    let customer = db.update_customer_by_id(*id, updated_customer.into_inner());
    match customer {
        Some(customer) => HttpResponse::Ok().json(customer),
        None => HttpResponse::NotFound().body("Customer not found"),
    }
}

/// Delete customer by id
#[delete("/customers/{id}")]
// pub async fn delete_customer_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
pub async fn delete_customer_by_id(db: web::Data<MySQLBackend>, id: web::Path<String>) -> HttpResponse {
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
