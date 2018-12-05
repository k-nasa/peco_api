use super::super::schema::*;
use crate::diesel::{pg::PgConnection, prelude::*};

#[derive(Clone, Queryable, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedPhras {
    id: i32,
    user_id: i32,
    pub yes_text: String,
    pub no_text: String,
}

impl FixedPhras {
    pub fn find_by_user_id(conn: &PgConnection, user_id: i32) -> Option<FixedPhras> {
        let fixed_phras = fixed_phrases::table
            .filter(fixed_phrases::user_id.eq(user_id))
            .limit(1)
            .load::<FixedPhras>(conn)
            .expect("Error loading users");

        if fixed_phras.is_empty() {
            return None;
        }

        let fixed_phras = fixed_phras.first().unwrap();

        Some(fixed_phras.clone())
    }

    pub fn update_yes_text(
        &self,
        conn: &PgConnection,
        text: &str,
    ) -> Result<FixedPhras, diesel::result::Error> {
        diesel::update(fixed_phrases::table.filter(fixed_phrases::id.eq(self.id)))
            .set(fixed_phrases::yes_text.eq(text))
            .get_result(conn)
    }

    pub fn update_no_text(
        &self,
        conn: &PgConnection,
        text: &str,
    ) -> Result<FixedPhras, diesel::result::Error> {
        diesel::update(fixed_phrases::table.filter(fixed_phrases::id.eq(self.id)))
            .set(fixed_phrases::no_text.eq(text))
            .get_result(conn)
    }
}
