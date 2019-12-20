
/* 
    Url route should be using attribute
    #[get("toute_name")] 

    Rockets requests: get, posts, head, etc
*/

use crate::config;
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;

use crate::models;
use models::user_model::User;
use diesel::result::Error;

#[get("/")]
pub fn index() -> &'static str {
    "Application Successfully Started"
}

#[get("/users")]
pub fn view_users(connection: config::Connection) -> JsonValue {
    let result = User::view_users(&connection);
    json!({
        "users": result
    })
}

#[post("/users", data = "<user>")] 
pub fn create_user(user: Json<User>, connection: config::Connection) -> Json<User> {
    let new_user = user.into_inner();
    return Json(User::create_user(new_user, &connection))
}
