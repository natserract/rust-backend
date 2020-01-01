use crate::schema::issues;
use chrono::NaiveDateTime;

use crate::models::users;
use users::User;

use rocket_contrib::json::{Json, JsonValue};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct Issues {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub author: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[table_name = "issues"]
#[derive(Serialize, Deserialize, Insertable, Queryable)]
pub struct NewIssues<'a> {
    #[serde(skip_deserializing)]
    pub slug: &'a str,
    pub title: &'a str,
    pub body: &'a str,

    #[serde(skip_deserializing)]
    pub author: i32,

    #[serde(skip_deserializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_deserializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[table_name = "issues"]
#[derive(Serialize, Deserialize, AsChangeset, Clone)]
pub struct UpdateIssues {
    #[serde(skip)]
    pub slug: Option<String>,
    pub title: Option<String>,
    pub body: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
