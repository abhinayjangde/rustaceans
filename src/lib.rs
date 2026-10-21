#![allow(dead_code, unused_variables)]
mod database;
pub mod auth_util;

use crate::auth_util::login;
use database::{Status, connect_to_database};


pub fn authenticate(cred: auth_util::models::Credentials) {
    if let Status::Connected = connect_to_database() {
        println!("db connected");
        login(cred);
    }
}