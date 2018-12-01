extern crate peco_api;
use peco_api::rocket;

fn main() {
    rocket().launch();
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
