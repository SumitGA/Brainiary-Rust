#[macro_use]
extern crate diesel;

mod resume;
mod auth;
mod db;
mod schema;
mod health;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use actix_web::http::header;
use actix_governor::{Governor, GovernorConfigBuilder};
use db::establish_connection;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool = establish_connection();

    // Rate limiting configuration: 60 requests per minute (1 req/sec)
    let governor_conf = GovernorConfigBuilder::default()
        .seconds_per_request(1) // 1 second per request = 60 req/min
        .burst_size(10)
        .finish()
        .unwrap();

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
        .wrap(Governor::new(&governor_conf))
        .configure(health::init_routes)
        .configure(resume::handler::init_routes)
        .configure(auth::handler::init_routes)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}