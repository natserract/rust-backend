use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

use crate::schema::users;
use crate::schema::users::dsl::users as all_users;

use crate::models;
use models::user_model::{NewUser, UpdateUser, User};

use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};

pub fn query_view_all_users(connection: &MysqlConnection) -> Vec<User> {
    users::table
        .order(users::id.desc())
        .load::<User>(connection)
        .unwrap()
}

pub fn query_find_user_by_id(user_id: i32, connection: &MysqlConnection) -> Option<User> {
    users::table
        .find(user_id)
        .get_result(connection)
        .map_err(|err| println!("find_user: {}", err))
        .ok()
}


pub fn query_create_user(
    connection: &MysqlConnection,
    name: &str,
    email: &str,
    password_param: &str,
) -> User {
    // Password hashing <-
    let params = ScryptParams::new(16, 4, 1);
    let password = &scrypt_simple(password_param, &params).unwrap_or("".to_string());

    let new_user = &NewUser {
        name,
        email,
        password,
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .execute(connection)
        .expect("Failed to create user");

    users::table
        .order(users::id.desc())
        .first(connection)
        .unwrap()
}

pub fn query_update_user(
    user_id: i32,
    user_field: UpdateUser,
    connection: &MysqlConnection,
) -> bool {
    let data = &UpdateUser {
        password: None,
        ..user_field.clone()
    };

    diesel::update(users::table.find(user_id))
        .set(data)
        .execute(connection)
        .is_ok()
}

pub fn query_delete_user(user_id: i32, connection: &MysqlConnection) -> bool {
    diesel::delete(users::table.find(user_id))
        .execute(connection)
        .is_ok()
}

pub fn query_delete_all_user(connection: &MysqlConnection) -> bool {
    diesel::delete(all_users).execute(connection).is_ok()
}
