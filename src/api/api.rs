//! Define the REST API for CRUD operations on our MySQL db

use actix_web::{delete, get, post, put, web};
use actix_web::{web::{
    Data,
    Json,
}, HttpResponse};
use serde::Serialize;
use crate::backend::MySQLBackend;

use crate::models::customer::mysql::Customer;

// REST: API:
// Customer:
//      GET customer/{id}
//      GET customers
//      PUT (JSON BODY) customers/{id}
//      POST (JSON BODY) customers
// Employee:
//      GET employee/{id}
//      GET employees
//      PUT (JSON BODY) employees/{id}
//      POST (JSON BODY) employees
// Work Orders:
//      GET work-order/{id}

// Customer Endpoints:

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
pub async fn delete_customer_by_id(db: web::Data<MySQLBackend>, id: web::Path<i32>) -> HttpResponse {

    let customer = db.delete_customer_by_id(*id);
    match customer {
        Some(customer) => HttpResponse::Ok().json(customer),
        None => HttpResponse::NotFound().body("Customer not found"),
    }
}

// Employee Endpoints

// Helper Functions
#[derive(Serialize)]
pub struct Response {
    pub message: String,
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
