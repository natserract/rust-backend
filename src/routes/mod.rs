

use rocket_contrib::json::{
    Json, 
    JsonValue
};

pub mod user_route;
pub mod post_route;

// -> Handle Not Found Routes
#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "Error",
        "reason": "Resource was not found"
    })
}