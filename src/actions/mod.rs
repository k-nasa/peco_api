pub mod fixed_phras;
pub mod send_email;
pub mod user;

use crate::diesel::{pg::PgConnection, prelude::*};
use dotenv::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().expect("faild load dotenv");

    let databese_url = env::var("DATABASE_URL").expect("DATABASE_URL mut be set");

    PgConnection::establish(&databese_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", databese_url))
}
