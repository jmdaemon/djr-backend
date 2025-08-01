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

diesel::table! {
    employees (employee_id) {
        employee_id -> Integer,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 12]
        phone -> Varchar,
        #[max_length = 50]
        email -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    employees,
);
