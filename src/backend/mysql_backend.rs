use std::env;
use std::fmt::Error;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::mysql::Customer;
use crate::models::schema::customers::dsl::*;

use crate::models::mysql::Employee;
use crate::models::schema::employees::dsl::*;

// use crate::api::API;

pub type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct MySQLBackend {
    pool: DBPool,
}

impl MySQLBackend {
    pub fn new() -> Self {

        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        Self { pool }
    }

    // Customer Endpoints:

    pub fn get_customers(&self) -> Vec<Customer> {
        customers.load::<Customer>(&mut self.pool.get().unwrap())
            .expect("Error loading all customers")
    }

    pub fn get_customer_by_id(&self, id: i32) -> Option<Customer> {
        let customer = customers
            .find(id)
            .get_result::<Customer>(&mut self.pool.get().unwrap())
            .expect("Error loading customer by id");
        Some(customer)
    }

    pub fn create_customer(&self, customer: Customer) -> Result<Customer, Error> {
        let id = self.get_customers().len() as i32;
        let customer = Customer {
            customer_id: id,
            ..customer
        };

        diesel::insert_into(customers)
            .values(&customer)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new customer");
        Ok(customer)
    }

    pub fn update_customer_by_id(&self, id: i32, mut customer: Customer) -> Option<Customer> {
        // Replace the customer in the db with the new customer data
        let _ = diesel::update(customers.find(id))
            .set(&customer)
            .execute(&mut self.pool.get().unwrap());

        let customer =customers 
            .find(id)
            .get_result::<Customer>(&mut self.pool.get().unwrap())
            .expect("Error loading customer by id");
        Some(customer)
    }
    
    pub fn delete_customer_by_id(&self, id: i32) -> Option<usize> {
        let count = diesel::delete(customers.find(id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting customer by id");
        Some(count)
    }

    // Employee Endpoints:

    pub fn get_employees(&self) -> Vec<Employee> {
        employees.load::<Employee>(&mut self.pool.get().unwrap())
            .expect("Error loading all employees")
    }

    pub fn get_employee_by_id(&self, id: i32) -> Option<Employee> {
        let employee = employees
            .find(id)
            .get_result::<Employee>(&mut self.pool.get().unwrap())
            .expect("Error loading employee by id");
        Some(employee)
    }

    pub fn create_employee(&self, employee: Employee) -> Result<Employee, Error> {
        let id = self.get_customers().len() as i32;
        let employee  = Employee{
            employee_id: id,
            ..employee
        };

        diesel::insert_into(employees)
            .values(&employee)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new employee");
        Ok(employee)
    }

    pub fn update_employee_by_id(&self, id: i32, mut employee: Employee) -> Option<Employee> {
        // Replace the customer in the db with the new customer data
        let _ = diesel::update(employees.find(id))
            .set(&employee)
            .execute(&mut self.pool.get().unwrap());

        let employee = employees
            .find(id)
            .get_result::<Employee>(&mut self.pool.get().unwrap())
            .expect("Error loading employee by id");
        Some(employee)
    }
    
    pub fn delete_employee_by_id(&self, id: i32) -> Option<usize> {
        let count = diesel::delete(employees.find(id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting employee by id");
        Some(count)
    }


}
