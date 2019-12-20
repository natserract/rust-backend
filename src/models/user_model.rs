use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use crate::schema::user;
use crate::schema::user::dsl::user as all_users;

use serde::{Deserialize, Serialize};

#[table_name = "user"]
#[derive(Serialize, Insertable, Deserialize, AsChangeset, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub names: String,
}

impl User {
    pub fn view_users(connection: &MysqlConnection) -> Vec<User> {
        user::table
            .order(user::id.desc())
            .load::<User>(connection)
            .unwrap()
    }

    pub fn create_user(user: User, connection: &MysqlConnection) -> User {
        diesel::insert_into(user::table)
            .values(&user)
            .execute(connection)
            .expect("Failed to create user");

        user::table.order(user::id.desc())
            .first(connection)
            .unwrap()
    }
}
