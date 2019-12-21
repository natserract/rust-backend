
use crate::schema::user;
use crate::schema::user::dsl::user as all_users;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub names: String,
}

#[table_name = "user"]
#[derive(Serialize, Deserialize, Insertable, Queryable, AsChangeset)]
pub struct NewUser {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub names: String,
}

