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
    todos (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Longtext>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    todos,
);
