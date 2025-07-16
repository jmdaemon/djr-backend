use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

use crate::api::API;
use crate::models::customer::mock::Customer;

pub type Index = AtomicI32;
pub struct MockBackend {
    pub customers: Arc<Mutex<Vec<Customer>>>,
    pub index: Index,
}

impl MockBackend {
    pub fn new() -> Self {
        let customers = Arc::new(Mutex::new(vec![]));
        let index = AtomicI32::new(0);
        Self { customers, index }
    }
}

impl API for MockBackend {
    fn create_customer(&self, customer: Customer) -> Result<Customer, std::fmt::Error> {
        let mut customers = self.customers.lock().unwrap();

        // Increment Customer ID
        let mut i = self.index.load(Ordering::Relaxed);
        i += 1;
        self.index.store(i, Ordering::Relaxed);
        println!("{i}");

        let customer_id = i;

        let customer = Customer {
            customer_id: Some(customer_id),
            ..customer
        };
        customers.push(customer.clone());

        Ok(customer)
    }

    fn get_customers(&self) -> Vec<Customer> {
        self.customers.get_cloned().unwrap()
    }
}
