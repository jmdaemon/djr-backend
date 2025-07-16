#![feature(lock_value_accessors)]

pub mod api;
pub mod models;
pub mod repository;

use diesel::prelude::*;

use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
