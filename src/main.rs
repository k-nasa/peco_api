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

fn main() {
    rocket().launch();
}

use self::models::user::*;
use crate::diesel::{pg::PgConnection, prelude::*};
use crate::rocket_contrib::json::{Json, JsonValue};
use dotenv::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().expect("faild load dotenv");

    let databese_url = env::var("DATABASE_URL").expect("DATABASE_URL mut be set");

    PgConnection::establish(&databese_url).expect(&format!("Error connecting to {}", databese_url))
}

#[derive(Serialize, Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
    password_confirmation: String,
}

#[post("/users", format = "application/json", data = "<user>")]
pub fn post_users(user: Json<RequestUser>) -> JsonValue {
    let connection = establish_connection();

    // TODO validation and error handle
    let result_user = User::create_user(&connection, &user.username, &user.password).unwrap();

    json!({ "token": result_user.token })
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use std::time::SystemTime;

    #[test]
    fn post_users() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let now = format!("{:?}", SystemTime::now());

        let mut response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(format!(
                r#"{{
                  "username": "test_user{}",
                  "password": "password",
                  "password_confirmation": "password"
                  }}"#,
                now,
            ))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type().unwrap(), ContentType::JSON);
        assert!(response.body_string().unwrap().contains(r#"{"token":"#))
    }
}
