use crate::config;
use rocket_contrib::json::{Json, JsonValue};

use rocket::http::Status;

use crate::models;
use models::issues_model::{Issues, NewIssues, UpdateIssues};

use crate::database;
use database::issues_database as action;

#[get("/issues")]
pub fn route_view_all_issues(connection: config::Connection) -> JsonValue {
    let result = action::query_view_all_issues(&connection);
    json!({ "issues": result })
}

#[get("/issue/<issue_id>")]
pub fn route_find_user(issue_id: i32, connection: config::Connection) -> Option<JsonValue> {
    action::query_find_issue(issue_id, &connection)
        .map(|issues| json!({ "issues": issues }))
}

#[post("/issue/<author>", data = "<issue_data>")]
pub fn route_create_user(author: i32, issue_data: Json<NewIssues>, connection: config::Connection) -> Json<Issues> {
    let title = issue_data.title;
    let body = issue_data.body;
    let author = author;

    Json(action::query_create_issues(
        &connection, 
        title, 
        body, 
        author
    ))
}