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

pub fn query_find_issue(slug: &str, connection: &MysqlConnection) -> Option<Issues> {
    issues::table
        .filter(issues::slug.eq(slug))
        .get_result(connection)
        .map_err(|err| println!("Error: {}", err))
        .ok()
}

pub fn query_create_issue(
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
        false => panic!("Error! You must be login/register account"),
    };

    issues::table
        .order(issues::id.desc())
        .first(connection)
        .unwrap()
}

pub fn query_update_issue(
    author: i32,
    slug: &str,
    mut issue_data: UpdateIssues,
    connection: &MysqlConnection,
) -> bool {
    let updated_at = Some(Local::now().naive_local());

    // Find user
    let find_author = users::table
        .find(author)
        .get_result::<User>(connection)
        .is_ok();

    // Find article slug
    let find_slug = issues::table
        .filter(issues::slug.eq(slug))
        .get_result::<Issues>(connection)
        .is_ok();

    if let Some(ref title) = issue_data.title {
        issue_data.slug = Some(hooks::slugify(&title));
    }

    let data = &UpdateIssues {
        created_at: None,
        updated_at: updated_at,
        ..issue_data.clone()
    };

    if find_author == true && find_slug == true {
        diesel::update(issues::table.filter(issues::slug.eq(slug)))
            .set(data)
            .execute(connection)
            .is_ok()
    } else {
        panic!("Error! You haven't permission!")
    }
}
