use actix_web::{web, HttpRequest, HttpResponse, Responder};
use super::model::ResumeRequest;
use super::generator::generate_resume;

pub async fn generate_resume_handler(payload: web::Json<ResumeRequest>) -> impl Responder {
    match generate_resume(&payload).await {
        Ok(resume) => HttpResponse::Ok().json(serde_json::json!({"resume": resume})),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({"resume": "Error generating resume"}))
    }
}

pub fn init_routes(&mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/resume")
        .route("/create", web::post().to(resume::handlers::generate_resume))
    )
}