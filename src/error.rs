
use rocket_contrib::json::{ Json, JsonValue};

// -> Handle Not Found Routes
#[catch(404)]
pub fn error_status() -> JsonValue {
    json!({
        "status": "Error",
        "reason": "Resource was not found"
    })
}