use crate::schema::users;
use diesel::prelude::*;
use uuid::Uuid;
use crate::auth::model::User;
use chrono::Utc;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: Uuid,
    pub email: String,
    pub hashed_password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl NewUser {
    pub fn new(email: String, hashed_password: String) -> Self {
        let now = Utc::now().naive_utc();
        NewUser {
            id: Uuid::new_v4(),
            email,
            hashed_password,
            created_at: now,
            updated_at: now,
        }
    }
}