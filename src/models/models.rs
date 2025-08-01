// TODO: Implement Customer model via diesel backend

// #[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
// #[diesel(table_name = crate::models::schema::todos)]
// #[diesel(check_for_backend(diesel::mysql::Mysql))]
// pub struct Todo {
//     #[serde(default)]
//     pub id: String,
//     pub title: String,
//     pub description: Option<String>,
//     pub created_at: Option<chrono::NaiveDateTime>,
//     pub updated_at: Option<chrono::NaiveDateTime>,
// }

pub mod mock {
    use serde::{Deserialize, Serialize};
    use diesel::{Queryable, Insertable, AsChangeset};

    #[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
    pub struct Customer {
        pub customer_id: Option<i32>,
        pub first_name: String,
        pub last_name: String,
        pub phone: String,
        pub email: String,
    }
}

pub mod mysql {
    use serde::{Deserialize, Serialize};
    use diesel::{Queryable, Insertable, AsChangeset};

    #[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
    #[diesel(table_name = crate::models::schema::customers)]
    #[diesel(check_for_backend(diesel::mysql::Mysql))]
    pub struct Customer {
        #[serde(default)]
        pub customer_id: i32,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub phone: Option<String>,
        pub email: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
    #[diesel(table_name = crate::models::schema::employees)]
    #[diesel(check_for_backend(diesel::mysql::Mysql))]
    pub struct Employee {
        #[serde(default)]
        pub employee_id: i32,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub phone: Option<String>,
        pub email: Option<String>,
    }
}
