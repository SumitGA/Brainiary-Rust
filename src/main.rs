#[macro_use]
extern crate diesel;

mod resume;
mod auth;
mod db;
mod schema;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use actix_web::http::header;
use db::establish_connection;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
        .wrap(Logger::default())
        .wrap(
            Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "OPTIONS", "PUT", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
            .max_age(3600)
        )
        .app_data(web::Data::new(pool.clone()))
        .route("/api/v1/resume", web::post().to(resume::handlers::generate_resume_handler))
        .configure(auth::handler::init_routes)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}