// @generated automatically by Diesel CLI.

diesel::table! {
    customers (customer_id) {
        customer_id -> Integer,
        #[max_length = 50]
        first_name -> Nullable<Varchar>,
        #[max_length = 50]
        last_name -> Nullable<Varchar>,
        #[max_length = 12]
        phone -> Nullable<Varchar>,
        #[max_length = 50]
        email -> Nullable<Varchar>,
    }
}
