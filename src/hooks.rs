
use slug;
use rocket_contrib::json::{ Json, JsonValue};

use rand::{
    distributions::Alphanumeric,
    thread_rng,
    Rng
};

// -> Handle Not Found Routes
#[catch(404)]
pub fn error_status() -> JsonValue {
    json!({
        "status": "Error",
        "reason": "Resource was not found {}",
    })
}

pub fn slugify(title: &str) -> String {
    if cfg!(feature = "random-suffix") {
        format!("{}-{}-{}",  
            generate_suffix(9).to_lowercase(), 
            generate_suffix(4).to_lowercase(), 
            generate_suffix(10).to_lowercase())
    } else {
        slug::slugify(title)
    }
}

pub fn generate_suffix(len: usize)-> String {
    let mut rng = thread_rng();
    (0..len).map(|_| rng.sample(Alphanumeric)).collect()
}