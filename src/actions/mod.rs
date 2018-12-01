use super::models::user::*;
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
