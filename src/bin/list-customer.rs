//! Test Customer table integration

use diesel::prelude::*;

use djr_backend::models::todo::Customer;
use djr_backend::models::schema::Customer as schema_customer;

fn main() {
    let mut connection = djr_backend::establish_connection();

    let results = schema_customer::dsl::Customer
        .limit(5)
        .load::<Customer>(&mut connection)
        .expect("Error loading customers");

    println!("-----------------------------------------");
    println!("Found {} customers", results.len());
    println!("----------------------------------------\n");

    for c in results {
        println!("First Name : {:?}", c.first_name);
        println!("Last Name  : {:?}", c.last_name);
        println!("Phone      : {:?}", c.phone);
        println!("Email      : {:?}", c.email);
        println!("----------------------------------------\n");
    }
}
