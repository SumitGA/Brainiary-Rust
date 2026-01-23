use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub async fn get_health() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(get_health));
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().configure(init_routes)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["status"], "ok");
    }
}
