use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use crate::schema::issues;
use crate::schema::issues::dsl::issues as all_issues;
use crate::schema::users;

use diesel::result::{DatabaseErrorKind, Error};

use crate::models;
use models::issues_model::{Issues, NewIssues, UpdateIssues};
use models::user_model::User;

use crate::hooks;
use chrono::Local;

pub fn query_view_all_issues(connection: &MysqlConnection) -> Vec<Issues> {
    issues::table
        .order(issues::id.desc())
        .load::<Issues>(connection)
        .unwrap()
}

pub fn query_find_issue(issue_id: i32, connection: &MysqlConnection) -> Option<Issues> {
    issues::table
        .find(issue_id)
        .get_result(connection)
        .map_err(|err| println!("find_user: {}", err))
        .ok()
}

pub fn query_create_issues(
    connection: &MysqlConnection,
    title: &str,
    body: &str,
    author: i32,
) -> Issues {

    let created_at = Some(Local::now().naive_local());
    let new_issue = &NewIssues {
        slug: &hooks::slugify(title),
        title,
        body,
        author: author,
        created_at: created_at,
        updated_at: None,
    };

    // Find user
    let author = users::table
        .find(author)
        .get_result::<User>(connection)
        .is_ok();

    match author {
        true => diesel::insert_into(issues::table)
            .values(new_issue)
            .execute(connection)
            .ok(),
        false => panic!("Error"),
    };

    issues::table
        .order(issues::id.desc())
        .first(connection)
        .unwrap()
}