use std::fmt::Error;
use chrono::prelude::*;
use std::sync::{Arc, Mutex};

// use diesel::prelude::*;

use crate::models::todo::Todo;
// use crate::models::todo::Customer;
// use crate::models::schema::Customer::dsl::*;

pub struct Database {
    pub todos: Arc<Mutex<Vec<Todo>>>,
}

impl Database {
    pub fn new() -> Self {
        let todos = Arc::new(Mutex::new(vec![]));
        Database { todos }
    }

    pub fn create_todo(&self, todo: Todo) -> Result<Todo, Error> {
        let mut todos = self.todos.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let todo = Todo {
            id: Some(id),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            ..todo
        };
        todos.push(todo.clone());
        Ok(todo)
    }

    pub fn get_todos(&self) -> Vec<Todo> {
        let todos = self.todos.lock().unwrap();
        todos.clone()
    }

    pub fn get_todo_by_id(&self, id: &str) -> Option<Todo> {
        let todos = self.todos.lock().unwrap();
        todos.iter().find(|todo| todo.id == Some(id.to_string())).cloned()
    }

    pub fn update_todo_by_id(&self, id: &str, todo: Todo) -> Option<Todo> {
        let mut todos = self.todos.lock().unwrap();
        let updated_at = Utc::now();
        let created_at = todo.created_at;
        // TODO: For some reason, ..todo would not clone created_at timestamp field
        // to todos

        let todo = Todo {
            id: Some(id.to_string()),
            updated_at: Some(updated_at),
            created_at,
            ..todo
        };
        let index = todos.iter().position(|todo| todo.id == Some(id.to_string()))?;
        todos[index] = todo.clone();
        Some(todo)
    }

    pub fn delete_todo_by_id(&self, id: &str) -> Option<Todo> {
        let mut todos = self.todos.lock().unwrap();
        let index = todos.iter().position(|todo| todo.id == Some(id.to_string()))?;
        Some(todos.remove(index))
    }
}
