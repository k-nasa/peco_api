#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::rocket::Rocket;


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
