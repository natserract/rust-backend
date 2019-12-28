
use crate::schema::users;
use crate::schema::users::dsl::users as all_users;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String
}

#[table_name = "users"]
#[derive(Serialize, Deserialize, Insertable, Queryable, AsChangeset)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str
}


#[table_name = "users"]
#[derive(Deserialize, Default, AsChangeset, Clone)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    
    #[column_name = "password"]
    pub password: Option<String>
}