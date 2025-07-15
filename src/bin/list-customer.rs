// use djr_backend::*;
// use self::models::*;
// use diesel::prelude::*;

use diesel::prelude::*;

use djr_backend::models::todo::Customer;
use djr_backend::models::schema::Customer as CustomerSchema;

// use djr_backend::repository::schema::customer::dsl::*;

//use djr_backend::models::schema as CustomerSchema;

// fn main() {
//     let connection = djr_backend::establish_connection();

//     let results = CustomerSchema::Customer

fn main() {
    let mut connection = djr_backend::establish_connection();

    // let results = Customer
    let results = CustomerSchema::dsl::Customer
        .limit(5)
        .load::<Customer>(&mut connection)
        .expect("Error loading posts");

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
