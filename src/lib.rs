#![feature(proc_macro_hygiene, decl_macro)]

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

use self::actions::*;
use crate::rocket::Rocket;

pub fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![post_users])
}
