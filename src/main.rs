// This line must be in the crate root - main.rs
#![feature(proc_macro_hygiene, decl_macro, plugin)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate chrono;

mod routes;
mod models;
mod schema;
mod config;
mod database;
mod hooks;

fn main() {
   config::connect_db().launch();
}
