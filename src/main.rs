// This line must be in the crate root - main.rs
#![feature(proc_macro_hygiene, decl_macro, plugin)]
#![allow(dead_code)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

mod routes;
mod models;
mod schema;
mod config;

fn main() {
   config::connect_db().launch();
}
