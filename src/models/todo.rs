use chrono::prelude::{DateTime, Utc};

use serde::{Deserialize, Serialize};

use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::models::schema::todos)]
pub struct Todo {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}


#[derive(Queryable)]
pub struct Customer {
    pub customer_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
}
