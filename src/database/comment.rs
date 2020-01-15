
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use crate::schema::issues;
use crate::schema::issues::dsl::issues as all_issues;
use crate::schema::users;

use diesel::result::{DatabaseErrorKind, Error};

use crate::models;
use models::issues::{Issues, NewIssues, UpdateIssues};
use models::users::User;

use crate::hooks;
use chrono::Local;

