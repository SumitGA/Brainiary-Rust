use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};

use crate::auth::model::{RegisterInput, LoginInput, AuthResponse, User};
use crate::auth::generator::NewUser;
use crate::schema::users::dsl::*;
use crate::db::PgPool;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn register_user(
    pool: web::Data<PgPool>,
    input: web::Json<RegisterInput>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let hashed = hash(&input.password, DEFAULT_COST).expect("Failed to hash password");
    let new_user = NewUser::new(input.email.clone(), hashed);

    let result = diesel::insert_into(users)
        .values(&new_user)
        .get_result::<User>(conn);

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

pub async fn login_user(
    pool: web::Data<PgPool>,
    input: web::Json<LoginInput>,
) -> impl Responder {
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let user_result = users
        .filter(email.eq(&input.email))
        .first::<User>(conn);

    match user_result {
        Ok(user) => {
            if verify(&input.password, &user.hashed_password).unwrap_or(false) {
                let claims = Claims {
                    sub: user.email.clone(),
                    exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
                };
                let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());
                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(secret.as_ref()),
                )
                .expect("JWT token creation failed");
                let auth_response = AuthResponse::from_user(&user, token);

                HttpResponse::Ok().json(auth_response)
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Err(_) => HttpResponse::Unauthorized().body("User not found"),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/auth")
            .route("/register", web::post().to(register_user))
            .route("/login", web::post().to(login_user)),
    );
}