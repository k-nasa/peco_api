#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

pub mod actions;
pub mod models;
pub mod schema;

use self::actions::user::*;
use crate::rocket::Rocket;

pub fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![post_users, get_user_token])
}

fn main() {
    rocket().launch();
}
