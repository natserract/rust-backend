
use crate::schema::users;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, AsChangeset, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,

    #[serde(skip_serializing)] 
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}

#[table_name = "users"]
#[derive(Serialize, Deserialize, Insertable, Queryable)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,

    #[serde(skip_serializing)]
    pub password: &'a str,

    #[serde(skip_deserializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_deserializing)]
    pub updated_at: Option<NaiveDateTime>
}


#[table_name = "users"]
#[derive(Serialize, Deserialize, AsChangeset, Clone)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    
    #[column_name = "password"]
    pub password: Option<String>,

    #[serde(skip_deserializing)]
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>
}