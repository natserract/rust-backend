use crate::config;
use rocket_contrib::json::{Json, JsonValue};

use rocket::http::Status;

use crate::models;
use models::user_model::NewUser;

use crate::database;
use database::user_database as query;

#[get("/")]
pub fn index() -> String {
    format!("Application sucessfully started")
}

#[get("/users")]
pub fn view_users(connection: config::Connection) -> JsonValue {
    let result = query::view_users(&connection);
    json!({ "users": result })
}

#[post("/users", data = "<user>")]
pub fn create_user(user: Json<NewUser>, connection: config::Connection) -> Json<NewUser> {
    let new_user = user.into_inner();

    return Json(query::create_user(new_user, &connection));
}

#[delete("/users/<id>")]
pub fn delete_user(id: i32, connection: config::Connection) -> JsonValue {
    let result = query::delete_user(id, &connection);
    json!({ "users": result })
}