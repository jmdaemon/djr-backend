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
    Customer,
    todos,
);
