// @generated automatically by Diesel CLI.

diesel::table! {
    Customer (CustomerID) {
        CustomerID -> Integer,
        #[max_length = 50]
        FirstName -> Varchar,
        #[max_length = 50]
        LastName -> Varchar,
        #[max_length = 12]
        Phone -> Varchar,
        #[max_length = 50]
        Email -> Varchar,
    }
}
