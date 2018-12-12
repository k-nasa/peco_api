use super::super::schema::*;
use crate::diesel::{pg::PgConnection, prelude::*};

#[derive(Debug)]
pub enum ValidationError {
    NotSetUserId,
    NotSetSendAddress,
    DatabaseError(diesel::result::Error),
}

#[derive(Clone, Queryable, Debug, PartialEq, Serialize, Deserialize)]
pub struct SendEmail {
    pub id: i32,
    pub user_id: i32,
    pub email: String,
}

impl ToString for ValidationError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl SendEmail {
    pub fn create(
        conn: &PgConnection,
        user_id: &i32,
        send_address: &str,
    ) -> Result<Self, ValidationError> {
        if send_address.is_empty() {
            return Result::Err(ValidationError::NotSetUserId);
        }

        // TODO add email format validation

        diesel::insert_into(send_emails::table)
            .values((
                send_emails::user_id.eq(user_id),
                send_emails::email.eq(send_address),
            ))
            .returning((send_emails::id, send_emails::user_id, send_emails::email))
            .get_result(conn)
            .map_err(ValidationError::DatabaseError)
    }
}
