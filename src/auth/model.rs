use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use diesel::Insertable;

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct RegisterInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub id: Uuid,
    pub email: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl AuthResponse {
    pub fn from_user(user: &User, token: String) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            token, 
            created_at: user.created_at, 
            updated_at: user.updated_at,
        }
    }
}