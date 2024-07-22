use actix_web::{web, HttpResponse, HttpServer};

pub fn admin_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("haha app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
