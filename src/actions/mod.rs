use super::models::user::*;
use crate::diesel::{pg::PgConnection, prelude::*};
use crate::rocket::{http::Status, response::*};
use crate::rocket_contrib::json::{Json, JsonValue};
use dotenv::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().expect("faild load dotenv");

    let databese_url = env::var("DATABASE_URL").expect("DATABASE_URL mut be set");

    PgConnection::establish(&databese_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", databese_url))
}

#[derive(Serialize, Deserialize)]
pub struct RequestUser {
    username: String,
    password: String,
    password_confirmation: String,
}

#[post("/users", format = "application/json", data = "<user>")]
pub fn post_users(user: Json<RequestUser>) -> status::Custom<JsonValue> {
    let connection = establish_connection();

    // TODO validation and error handle
    let result_user = match User::create_user(
        &connection,
        &user.username,
        &user.password,
        &user.password_confirmation,
    ) {
        Ok(user) => user,
        Err(e) => {
            return status::Custom(Status::BadRequest, json!({ "error": e.to_string() }));
        }
    };

    status::Custom(Status::Ok, json!({ "token": result_user.token }))
}

#[derive(Serialize, Deserialize)]
pub struct RequestGetToken {
    username: String,
    password: String,
}

#[post("/user_token", format = "application/json", data = "<user>")]
pub fn get_user_token(user: Json<RequestGetToken>) -> status::Custom<JsonValue> {
    let connection = establish_connection();

    let token = match User::get_token(&connection, &user.username, &user.password) {
        Some(token) => token,
        None => {
            println!("test");
            return status::Custom(
                Status::BadRequest,
                json!({ "message": "invalid username or password" }),
            );
        }
    };

    status::Custom(Status::Ok, json!({ "token": token }))
}

#[cfg(test)]
mod test {
    use crate::rocket;
    use crate::rocket::http::{ContentType, Status};
    use crate::rocket::local::Client;
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

    #[test]
    fn post_users_when_invalid_password() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let now = format!("{:?}", SystemTime::now());

        let mut response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(format!(
                r#"{{
                  "username": "test_user{}",
                  "password": "password1",
                  "password_confirmation": "password2"
                  }}"#,
                now,
            ))
            .dispatch();

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.content_type().unwrap(), ContentType::JSON);
        assert!(response
            .body_string()
            .unwrap()
            .contains(r#"{"error":"IncorrectPassword""#))
    }

    #[test]
    fn post_users_when_empty_username() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let now = format!("{:?}", SystemTime::now());

        let mut response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(format!(
                r#"{{
                  "username": "",
                  "password": "password",
                  "password_confirmation": "password"
                  }}"#
            ))
            .dispatch();

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.content_type().unwrap(), ContentType::JSON);
        assert!(response
            .body_string()
            .unwrap()
            .contains(r#"{"error":"NoUsernameSet""#))
    }

    #[test]
    fn post_users_when_empty_password() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let now = format!("{:?}", SystemTime::now());

        let mut response = client
            .post("/users")
            .header(ContentType::JSON)
            .body(format!(
                r#"{{
                  "username": "test_user{}",
                  "password": "",
                  "password_confirmation": "password2"
                  }}"#,
                now,
            ))
            .dispatch();

        assert_eq!(response.status(), Status::BadRequest);
        assert_eq!(response.content_type().unwrap(), ContentType::JSON);
        assert!(response
            .body_string()
            .unwrap()
            .contains(r#"{"error":"NoPasswordSet""#))
    }
}
