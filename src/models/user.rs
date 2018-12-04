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

impl ToString for AuthenticationError {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Queryable, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub token: String,
    pub username: String,
    pub password_digest: String,
}

impl User {
    pub fn create_user(
        conn: &PgConnection,
        username: &str,
        password: &str,
        password_confirmation: &str,
    ) -> Result<User, AuthenticationError> {
        let hashed_password = hash(password, DEFAULT_COST)?;
        let token = hash(username, DEFAULT_COST)?;

        if password.is_empty() {
            return Result::Err(AuthenticationError::NoPasswordSet);
        };

        if username.is_empty() {
            return Result::Err(AuthenticationError::NoUsernameSet);
        };

        if password != password_confirmation || password.len() < 6 {
            return Result::Err(AuthenticationError::IncorrectPassword);
        };

        diesel::insert_into(users::table)
            .values((
                users::token.eq(token),
                users::username.eq(username),
                users::password_digest.eq(hashed_password),
            ))
            .returning((
                users::id,
                users::token,
                users::username,
                users::password_digest,
            ))
            .get_result(conn)
            .map_err(AuthenticationError::DatabaseError)
    }

    pub fn get_token(conn: &PgConnection, username: &str, password: &str) -> Option<String> {
        let user = users::table
            .filter(users::username.eq(username))
            .limit(1)
            .load::<User>(conn)
            .expect("Error loading users");

        if user.is_empty() {
            return None;
        }

        let user = user.first().unwrap();
        if !verify(password, &user.password_digest).unwrap() {
            return None;
        }

        let token = user.token.clone();

        Some(token)
    }

    pub fn create_initial_fixed_phrases(&self, conn: &PgConnection) {
        match diesel::insert_into(fixed_phrases::table)
            .values(fixed_phrases::user_id.eq(self.id))
            .execute(conn)
        {
            Ok(_) => (),
            Err(e) => println!("faild create_initial_fixed_phrases: {}", e),
        }
    }

    pub fn find_by_token(conn: &PgConnection, token: &str) -> Option<User> {
        let user = users::table
            .filter(users::token.eq(token))
            .limit(1)
            .load::<User>(conn)
            .expect("Error loading users");

        if user.is_empty() {
            return None;
        }

        let user = user.first().unwrap();

        Some(*user)
    }
}
