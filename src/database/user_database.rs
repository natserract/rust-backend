use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use crate::schema::user;
use crate::schema::user::dsl::user as all_users;

use crate::models;
use models::user_model::{ User, NewUser };

pub fn view_users(connection: &MysqlConnection) -> Vec<User> {
    user::table
        .order(user::id.desc())
        .load::<User>(connection)
        .unwrap()
}

pub fn create_user(user: NewUser, connection: &MysqlConnection) -> NewUser {
    diesel::insert_into(user::table)
        .values(&user)
        .execute(connection)
        .expect("Failed to create user");

    user::table
        .order(user::id.desc())
        .first(connection)
        .unwrap()
}

pub fn delete_user(user_id: i32, connection: &MysqlConnection) -> bool{
    diesel::delete(user::table.find(user_id)).execute(connection).is_ok()
}