use crate::config;
use rocket_contrib::json::{Json, JsonValue};

use rocket::http::Status;

use crate::models;
use models::user_model::{ NewUser };

use crate::database;
use database::user_database as query;


#[get("/users")]
pub fn view_all_users(connection: config::Connection) -> JsonValue {
    let result = query::view_all_users(&connection);
    json!({ "users": result })
}

#[get("/users/<user_id>")]
pub fn find_user_by_id(user_id: i32, connection: config::Connection) -> Option<JsonValue> {
    query::find_user_by_id(user_id, &connection).map(|user| json!({
        "users": user
    }))
}

#[post("/users", data = "<user_field>")]
pub fn create_user(user_field: Json<NewUser>, connection: config::Connection) -> Json<NewUser> {
    let new_user = user_field.into_inner();
    Json(query::create_user(new_user, &connection))
}

#[put("/users/<user_id>", data = "<user_field>")]
pub fn update_user(user_id: i32, user_field: Json<NewUser>, connection: config::Connection) -> String {
    let result = user_field.into_inner();
    let query = query::update_user(user_id, result, &connection);
    format!("{}", query)
}

#[delete("/users/<user_id>")]
pub fn delete_user(user_id: i32, connection: config::Connection) -> String {
    let result = query::delete_user(user_id, &connection);
    format!("{}", result)
}
