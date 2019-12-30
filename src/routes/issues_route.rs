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

#[get("/issue/<slug>")]
pub fn route_find_issue(slug: String, connection: config::Connection) -> Option<JsonValue> {
    action::query_find_issue(&slug, &connection)
        .map(|issues| json!({ "issues": issues }))
}

#[post("/issue/<author>", data = "<issue_data>")]
pub fn route_create_issue(author: i32, issue_data: Json<NewIssues>, connection: config::Connection) -> Json<Issues> {
    let title = issue_data.title;
    let body = issue_data.body;
    let author = author;

    Json(action::query_create_issue(
        &connection, 
        title, 
        body, 
        author
    ))
}

#[put("/issue/<author>/<slug>", data = "<issue_data>")]
pub fn route_update_issue(
    author: i32,
    slug: String,
    issue_data: Json<UpdateIssues>,
    connection: config::Connection,
) -> String {
    let query = action::query_update_issue(author, &slug, issue_data.into_inner(), &connection);
    match query {
        true => format!("Issue Post has been succesfully updated"),
        false => format!("Failed to update issue post"),
    }
}