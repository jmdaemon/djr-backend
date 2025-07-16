use std::sync::{Arc, Mutex};

use crate::api::API;
use crate::models::todo::Customer;

pub struct MockBackend {
    pub customers: Arc<Mutex<Vec<Customer>>>,
}

impl MockBackend {
    pub fn new() -> Self {
        let customers = Arc::new(Mutex::new(vec![]));
        Self { customers }
    }
}

impl API for MockBackend {
    fn get_customers(&self) -> Vec<Customer> {
        self.customers.get_cloned().unwrap()
    }
}
