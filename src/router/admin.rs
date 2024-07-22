use crate::api;
use actix_web::{web, HttpResponse, HttpServer};

pub fn admin_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/").service(
            // 后台首页
            web::scope("/create_menu").service(
                web::resource("")
                    .route(web::get().to(api::sys_menu::create_menu))
                    .name("admin.create_menu"),
            ),
        ),
    );
}
