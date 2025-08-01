//! Define the REST API for CRUD operations on our MySQL db

use actix_web::{delete, get, post, put, web, Responder};
use actix_web::{web::{
    Data,
    Json,
}, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::backend::MySQLBackend;

use crate::models::mysql::Customer;
use crate::models::mysql::Employee;

// REST: API:
// Customer:
//      GET customer/{id}
//      GET customers
//      PUT (JSON BODY) customers/{id}
//      POST (JSON BODY) customers
// Customer Extra:
//      GET customer/email/
// Employee:
//      GET employee/{id}
//      GET employees
//      PUT (JSON BODY) employees/{id}
//      POST (JSON BODY) employees
// Work Orders:
//      GET work-order/{id}

#[derive(Deserialize)]
pub struct CustomerInfo {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

// Customer Endpoints:

/// Retrieve all customers
#[get("/customers")]
pub async fn get_customers(db: web::Data<MySQLBackend>) -> HttpResponse {
    HttpResponse::Ok().json(db.get_customers())
}

/// Search for customers by their fields
#[get("/customer")]
// pub async fn get_customer(db: web::Data<MySQLBackend>, info: web::Query<CustomerInfo>) -> HttpResponse {
pub async fn search_customer(db: web::Data<MySQLBackend>, info: web::Query<CustomerInfo>) -> HttpResponse {
    HttpResponse::Ok().json(db.search_customer(info.0))
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

/// Retrieve all employees
#[get("/employees")]
pub async fn get_employees(db: web::Data<MySQLBackend>) -> HttpResponse {
    HttpResponse::Ok().json(db.get_employees())
}

/// Get employee by id
#[get("/employees/{id}")]
pub async fn get_employee_by_id(db: web::Data<MySQLBackend>, id: web::Path<i32>) -> HttpResponse {

    let employee = db.get_employee_by_id(*id);
    match employee {
        Some(employee) => HttpResponse::Ok().json(employee),
        None => HttpResponse::NotFound().body("Employee not found"),
    }
}

/// Create a new employee
#[post("/employees")]
pub async fn create_employee(db: Data<MySQLBackend>, new_employee: Json<Employee>) -> HttpResponse {
    let employee = db.create_employee(new_employee.into_inner());
    match employee {
        Ok(employee) => HttpResponse::Ok().json(employee),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/// Update employee by id
#[put("/employees/{id}")]
pub async fn update_employee_by_id(db: web::Data<MySQLBackend>, id: web::Path<i32>, updated_employee: web::Json<Employee>) -> HttpResponse {
    let employee = db.update_employee_by_id(*id, updated_employee.into_inner());
    match employee {
        Some(employee) => HttpResponse::Ok().json(employee),
        None => HttpResponse::NotFound().body("Customer not found"),
    }
}

/// Delete employee by id
#[delete("/employees/{id}")]
pub async fn delete_employee_by_id(db: web::Data<MySQLBackend>, id: web::Path<i32>) -> HttpResponse {

    let employee = db.delete_employee_by_id(*id);
    match employee {
        Some(employee) => HttpResponse::Ok().json(employee),
        None => HttpResponse::NotFound().body("Customer not found"),
    }
}

// Helper Functions

/// Stub an implementation until it is complete
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

// Add all the routes to our REST api service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        // Customers

        // CRUD:
        .service(create_customer)
        .service(get_customers)
        .service(get_customer_by_id)
        .service(update_customer_by_id)
        .service(delete_customer_by_id)

        // Extra:
        .service(search_customer)

        // Employees
        .service(create_employee)
        .service(get_employees)
        .service(get_employee_by_id)
        .service(update_employee_by_id)
        .service(delete_employee_by_id)
    );
}
