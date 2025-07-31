use std::env;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

use crate::models::customer::mysql::Customer;
use crate::models::schema::customers::dsl::*;

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

    pub fn get_customers(&self) -> Vec<Customer> {
        customers.load::<Customer>(&mut self.pool.get().unwrap())
            .expect("Error loading all customers")
    }
}
