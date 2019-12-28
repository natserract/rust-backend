use crate::config;
use rocket_contrib::json::{Json, JsonValue};

use rocket::http::Status;

use crate::models;
use models::user_model::{ User, NewUser, UpdateUser };

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
pub fn create_user(user_field: Json<NewUser>, connection: config::Connection) -> Json<User> {
    let new_user = user_field.into_inner();
    let name = new_user.name;
    let email = new_user.email;
    let password = new_user.password;

    Json(query::create_user(&connection, name, email, password))

}

#[put("/users/<user_id>", data = "<user_field>")]
pub fn update_user(user_id: i32, user_field: Json<UpdateUser>, connection: config::Connection) -> JsonValue {
    let query = query::update_user(user_id, user_field.into_inner(), &connection);
    json!({
        "users": query
    })
}

#[delete("/users/<user_id>")]
pub fn delete_user(user_id: i32, connection: config::Connection) -> String {
    let result = query::delete_user(user_id, &connection);
    format!("{}", result)
}
