use crate::schema::comments;
use chrono::NaiveDateTime;

use crate::models::users;
use users::User;

use rocket_contrib::json::{
    Json,
    JsonValue
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
pub struct Comments {
    pub id: i32,
    pub body: String,
    pub author: i32,
    pub issues: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
