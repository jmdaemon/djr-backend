//! Define the REST API for CRUD operations on our MySQL db

use std::fmt::Error;

use actix_web::{delete, get, put, post, web};
use actix_web::{web::{
    Data,
    Json,
}, HttpResponse};
use crate::models::todo::Customer;
use crate::{models::todo::Todo};

// API Routes

// Backend API for use in our database providers/implementors
pub trait API {
    fn get_customers(&self) -> Vec<Customer>;
    fn create_customer(&self, customer: Customer) -> Result<Customer, Error>;
}

// A database is anything that implements our backend
pub type Database = dyn API;

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
pub async fn get_customers(db: web::Data<Database>) -> HttpResponse {
    let todos = db.get_customers();
    HttpResponse::Ok().json(todos)
}

/*
#[post("/todos")]
pub async fn create_todo(db: Data<Database>, new_todo: Json<Todo>) -> HttpResponse {
    let todo = db.create_todo(new_todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/todos")]
pub async fn get_todos(db: web::Data<Database>) -> HttpResponse {
    let todos = db.get_todos();
    HttpResponse::Ok().json(todos)
}

#[get("/todos/{id}")]
pub async fn get_todo_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let todo = db.get_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[put("/todos/{id}")]
pub async fn update_todo_by_id(db: web::Data<Database>, id: web::Path<String>, updated_todo: web::Json<Todo>) -> HttpResponse {
    let todo = db.update_todo_by_id(&id, updated_todo.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let todo = db.delete_todo_by_id(&id);
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

// Add all the routes to our REST api service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
            .service(create_todo)
            .service(get_todos)
            .service(get_todo_by_id)
            .service(update_todo_by_id)
            .service(delete_todo_by_id)
    );
}
*/

// Add all the routes to our REST api service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
            .service(create_customer)
            .service(get_customers)
    );
}
