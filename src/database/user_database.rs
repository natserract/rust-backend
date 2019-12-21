use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use crate::schema::user;
use crate::schema::user::dsl::user as all_users;

use crate::models;
use models::user_model::{ User, NewUser };

pub fn view_all_users(connection: &MysqlConnection) -> Vec<User> {
    user::table
        .order(user::id.desc())
        .load::<User>(connection)
        .unwrap()
}

pub fn create_user(user_field: NewUser, connection: &MysqlConnection) -> NewUser {
    diesel::insert_into(user::table)
        .values(&user_field)
        .execute(connection)
        .expect("Failed to create user");

    user::table
        .order(user::id.desc())
        .first(connection)
        .unwrap()
}

pub fn update_user(user_id: i32, user_field: NewUser, connection: &MysqlConnection) -> String {
    let query = diesel::update(user::table.find(user_id)).set(&user_field).execute(connection).is_ok();
    match query {
        true => format!("User has been succesfully updated"),
        false => format!("User can't be updated"),
    }
} 

pub fn delete_user(user_id: i32, connection: &MysqlConnection) -> String{
    let query = diesel::delete(user::table.find(user_id)).execute(connection).is_ok();
    match query {
        true => format!("User has been succesfully deleted"),
        false => format!("User can't be deleted"),
    }
}