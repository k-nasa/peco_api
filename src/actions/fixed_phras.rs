use crate::actions::*;
use crate::models::{fixed_phras::*, user::*};
use crate::rocket::{http::Status, response::*};
use crate::rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUpdateFixedPhras {
    token: String,
    yes_text: String,
    no_text: String,
}

#[put("/fixed_phras", format = "application/json", data = "<params>")]
pub fn update_fixed_phras(params: Json<RequestUpdateFixedPhras>) -> status::Custom<JsonValue> {
    let connection = establish_connection();

    let user = match User::find_by_token(&connection, &params.token) {
        Some(user) => user,
        None => return status::Custom(Status::BadRequest, json!({"message": "invalid token" })),
    };

    let fixed_phras = match FixedPhras::find_by_user_id(&connection, user.id) {
        Some(f) => f,
        None => {
            return status::Custom(
                Status::NotFound,
                json!({"message": "faild: not find fixed phras" }),
            )
        }
    };

    let fixed_phras = fixed_phras
        .update_yes_text(&connection, &params.yes_text)
        .unwrap();

    let fixed_phras = fixed_phras
        .update_no_text(&connection, &params.no_text)
        .unwrap();

    status::Custom(
        Status::Ok,
        json!({ "message": "更新しました", "fixed_phras": fixed_phras }),
    )
}

#[cfg(test)]
mod tests {
    use crate::actions::*;
    use crate::models::{fixed_phras::*, user::*};
    use crate::rocket;
    use crate::rocket::http::{ContentType, Status};
    use crate::rocket::local::Client;
    use std::time::SystemTime;

    #[test]
    fn update_fixed_phras() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let username = format!("test_user{:?}", SystemTime::now());
        let connection = establish_connection();

        User::create_user(&connection, &username, "password", "password");
        let token = User::get_token(&connection, &username, "password").unwrap();
        let user = User::find_by_token(&connection, &token).unwrap();
        user.create_initial_fixed_phrases(&connection);

        let fixed_phras = FixedPhras::find_by_user_id(&connection, user.id).unwrap();

        assert_eq!(fixed_phras.yes_text, "必要なり".to_string());
        assert_eq!(fixed_phras.no_text, "不要なり".to_string());

        let mut response = client
            .put("/fixed_phras")
            .header(ContentType::JSON)
            .body(format!(
                r#"{{
                  "token": "{}",
                  "yes_text": "yes",
                  "no_text": "no"
                  }}"#,
                token
            ))
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type().unwrap(), ContentType::JSON);
        assert!(response
            .body_string()
            .unwrap()
            .contains(r#""message":"更新しました""#));

        let fixed_phras = FixedPhras::find_by_user_id(&connection, user.id).unwrap();
        assert_eq!(fixed_phras.yes_text, "yes".to_string());
        assert_eq!(fixed_phras.no_text, "no".to_string());
    }
}
