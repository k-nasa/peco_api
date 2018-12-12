use crate::actions::*;
use crate::models::{send_email::*, user::*};
use crate::rocket::{http::Status, response::*};
use crate::rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
pub struct RequestCreateSendEmail {
    token: String,
    email: String,
}

#[post("/send_emails", format = "application/json", data = "<send_email>")]
pub fn post_send_emails(send_email: Json<RequestCreateSendEmail>) -> status::Custom<JsonValue> {
    let connection = establish_connection();

    let result_user = match User::find_by_token(&connection, &send_email.token) {
        Some(user) => user,
        None => {
            return status::Custom(Status::BadRequest, json!({ "error": "invalid token" }));
        }
    };

    let send_email = match SendEmail::create(&connection, &result_user.id, &send_email.email) {
        Ok(s) => s,
        Err(e) => {
            return status::Custom(Status::BadRequest, json!({ "error": e.to_string() }));
        }
    };

    status::Custom(Status::Ok, json!({ "send_email": send_email }))
}

#[cfg(test)]
mod test {
    use crate::actions::establish_connection;
    use crate::models::user::*;
    use crate::rocket;
    use crate::rocket::http::{ContentType, Status};
    use crate::rocket::local::Client;
    use std::time::SystemTime;

    #[test]
    fn post_send_email() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let now = format!("{:?}", SystemTime::now());
        let user =
            User::create_user(&establish_connection(), &now, "password", "password").unwrap();

        let mut response = client
            .post("/send_emails")
            .header(ContentType::JSON)
            .body(format!(
                r#"{{
                  "token": "{}",
                  "email": "email"
                  }}"#,
                user.token,
            ))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type().unwrap(), ContentType::JSON);
        assert!(response
            .body_string()
            .unwrap()
            .contains(r#"{"send_email":"#))
    }
}
