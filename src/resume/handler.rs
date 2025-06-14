use actix_web::{web, HttpRequest, HttpResponse, Responder};
use super::model::ResumeRequest;
use crate::db::PgPool;
use super::generator::generate_resume;

pub async fn generate_resume_handler(pool: web::Data<PgPool>, payload: web::Json<ResumeRequest>) -> impl Responder {
    match generate_resume(&payload).await {
        Ok(resume) => HttpResponse::Ok().json(serde_json::json!({"resume": resume})),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({"resume": "Error generating resume"}))
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/resume")
        .route("/create", web::post().to(generate_resume_handler)), 
    );
}