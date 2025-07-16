// use std::env;
// use std::fmt::Error;

// use chrono::prelude::*;
// use diesel::prelude::*;
// use diesel::r2d2::{self, ConnectionManager};
// use dotenv::dotenv;

// use crate::api::API;
// use crate::models::todo::{Customer, Todo};
// use crate::models::schema::todos::dsl::*;

// Mock Backend


// MySQL Backend

// pub struct Database<T> {
//     data: T,
// }


// TODO: Stub the database interface using traits to implement our REST API
// and integrate with diesel

/*
pub type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {

        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        Database { pool }
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        todos
            .load::<Todo>(&mut self.pool.get().unwrap())
            .expect("Error loading all todos")
    }

    pub fn create_todo(&self, todo: Todo) -> Result<Todo, Error> {
        let todo = Todo {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..todo
        };
        diesel::insert_into(todos)
            .values(&todo)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new todo");
        Ok(todo)
    }

    pub fn get_todo_by_id(&self, todo_id: &str) -> Option<Todo> {
        let todo = todos
            .find(todo_id)
            .get_result::<Todo>(&mut self.pool.get().unwrap())
            .expect("Error loading todo by id");
        Some(todo)
    }

    pub fn delete_todo_by_id(&self, todo_id: &str) -> Option<usize> {
        let count = diesel::delete(todos.find(todo_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting todo by id");
        Some(count)
    }

    pub fn update_todo_by_id(&self, todo_id: &str, mut todo: Todo) -> Option<Todo> {
        todo.updated_at = Some(Utc::now().naive_utc());
        // let todo = diesel::update(todos.find(todo_id))
        //     .set(&todo)

            // .::<Todo>(&mut self.pool.get().unwrap())
            // .expect("Error updating todo by id");
        // let todo = diesel::update(todos.find(todo_id))
        //     .set(&todo)
        //     .get_result::<Todo>(&mut self.pool.get().unwrap())
        //     .expect("Error loading todo by id");
        
        // let todo = todos
        //     .find(todo_id)
            // .get_result::<Todo>(&mut self.pool.get().unwrap())
            // .expect("Error loading todo by id");

        // MySQL doesn't support insertion + retrieval like in Postgres
        // https://github.com/diesel-rs/diesel/issues/2535
        let _ = diesel::update(todos.find(todo_id)).set(&todo);

        let todo = todos
            .find(todo_id)
            .get_result::<Todo>(&mut self.pool.get().unwrap())
            .expect("Error loading todo by id");

        Some(todo)
    }
}
*/
