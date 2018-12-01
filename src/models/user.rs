use super::super::schema::*;
use crate::diesel::{pg::PgConnection, prelude::*};
use bcrypt::*;

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
pub struct User {
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
