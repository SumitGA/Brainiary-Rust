mod modules;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use actix_web::http::header;
use modules::resume::handlers::generate_resume_handler;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
        .wrap(
            Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS", "PUT", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
            .max_age(3600),
        )
        .route("/api/v1/resume", web::post().to(generate_resume_handler))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}