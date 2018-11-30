#![feature(proc_macro_hygiene, decl_macro)]

pub mod schema;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

use crate::diesel::{pg::PgConnection, prelude::*, Queryable};
use crate::rocket::Rocket;
use crate::rocket_contrib::json::{Json, JsonValue};
use bcrypt::*;
use dotenv::*;
use std::env;

use self::schema::*;

fn establish_connection() -> PgConnection {
    dotenv().expect("faild load dotenv");

    let databese_url = env::var("DATABASE_URL").expect("DATABASE_URL mut be set");

    PgConnection::establish(&databese_url).expect(&format!("Error connecting to {}", databese_url))
}

#[derive(Debug)]
pub enum AuthenticationError {
    IncorrectPassword,
    NoUsernameSet,
    NoPasswordSet,
    BcryptError(BcryptError),
    DatabaseError(diesel::result::Error),
}

impl From<BcryptError> for AuthenticationError {
    fn from(e: BcryptError) -> Self {
        AuthenticationError::BcryptError(e)
    }
}

#[derive(Queryable, Debug, PartialEq)]
struct User {
    pub id: i32,
    pub username: String,
    pub token: String,
}

impl User {
    pub fn create_user(
        conn: &PgConnection,
        username: &str,
        password: &str,
    ) -> Result<User, AuthenticationError> {
        let hashed_password = hash(password, DEFAULT_COST)?;
        let token = hash(username, DEFAULT_COST)?;

        diesel::insert_into(users::table)
            .values((
                users::username.eq(username),
                users::password_digest.eq(hashed_password),
                users::token.eq(token),
            ))
            .returning((users::id, users::username, users::token))
            .get_result(conn)
            .map_err(AuthenticationError::DatabaseError)
    }
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    #[test]
    fn post_users() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(
                r#"{
                  "username": "test_user",
                  "password": "password",
                  "password_confirmation": "password"
                  }"#,
            )
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type().unwrap(), ContentType::JSON);
        assert!(response.body_string().unwrap().contains(r#"{"token":"#))
    }
}
