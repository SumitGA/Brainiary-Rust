use actix_web::{web, dev::Payload, Error, FromRequest, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use diesel::prelude::*;
use futures_util::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::Insertable;
use crate::schema::users::{self};
use crate::schema::users::dsl::*;
use crate::db::PgPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Queryable)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hashed_password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub role: String,
}


#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: Uuid,
    pub email: String,
    pub hashed_password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub role: String,
}

impl NewUser {
    pub fn new(user_email: String, user_hashed_password: String) -> Self {
        let now = Utc::now().naive_utc();
        NewUser {
            id: Uuid::new_v4(),
            email: user_email,
            hashed_password: user_hashed_password,
            created_at: now,
            updated_at: now,
            role: "user".to_string(),
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Admin, 
    User
}

#[derive(Debug, Serialize)]
pub struct AuthenticatedUser{
    pub id: Uuid,
    pub email: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub role: String,
}

impl AuthenticatedUser {
    pub fn from_user(user: &User, token: String) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            token, 
            created_at: user.created_at, 
            updated_at: user.updated_at,
            role: user.role.clone(),
        }
    }

    pub fn has_role(&self, required: &str) -> bool {
        self.role == required
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }

    pub fn is_user(&self) -> bool {
        self.has_role("user")
    }

    pub fn can(&self, action: &str, resource: &str) -> bool {
        match(self.role.as_str(), action, resource) {
            ("admin",_,_) => true, 
            ("user", "read", "profile") => true,
            _ => false,
        }
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization").cloned();

        if let Some(header_value) = auth_header {
            if let Ok(auth_str) = header_value.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());

                    if let Ok(token_data) = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(secret.as_bytes()),
                        &Validation::default(),
                    ) {
                        let pool = req.app_data::<web::Data<PgPool>>().cloned().unwrap();
                        let conn = &mut pool.get().expect("DB connection");

                        if let Ok(user_id) = Uuid::parse_str(&token_data.claims.sub) {
                            if let Ok(user) = users.filter(id.eq(user_id)).first::<User>(conn) {
                                return ready(Ok(AuthenticatedUser {
                                    id: user.id,
                                    email: user.email.to_string(),
                                    token: token.to_string(),
                                    created_at: user.created_at,
                                    updated_at: user.updated_at,
                                    role: user.role,
                                }));
                            }
                        }
                    }
                }
            }
        }

        ready(Err(actix_web::error::ErrorUnauthorized("Invalid or missing token")))
    }
}