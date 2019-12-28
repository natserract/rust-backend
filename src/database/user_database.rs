use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use crate::schema::users;

use crate::models;
use models::user_model::{NewUser, UpdateUser, User};

use crypto::scrypt::{scrypt_check, scrypt_simple, ScryptParams};

pub fn view_all_users(connection: &MysqlConnection) -> Vec<User> {
    users::table
        .order(users::id.desc())
        .load::<User>(connection)
        .unwrap()
}

pub fn find_user_by_id(user_id: i32, connection: &MysqlConnection) -> Option<User> {
    users::table
        .find(user_id)
        .get_result(connection)
        .map_err(|err| println!("find_user: {}", err))
        .ok()
}

pub fn create_user(
    connection: &MysqlConnection,
    name: &str,
    email: &str,
    password_param: &str,
) -> User {
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

pub fn update_user(user_id: i32, user_field: UpdateUser, connection: &MysqlConnection) -> User {
    let data = &UpdateUser {
        password: None,
        ..user_field.clone()
    };

    let query = diesel::update(users::table.find(user_id))
        .set(data)
        .execute(connection)
        .is_ok();

    match query {
        true => users::table.find(user_id).get_result(connection).unwrap(),
        false => users::table
            .find(user_id)
            .get_result(connection)
            .map_err(|err| println!("Failed: {}", err))
            .unwrap(),
    }
}

pub fn delete_user(user_id: i32, connection: &MysqlConnection) -> String {
    let query = diesel::delete(users::table.find(user_id))
        .execute(connection)
        .is_ok();
    match query {
        true => format!("User has been succesfully deleted"),
        false => format!("User can't be deleted"),
    }
}
