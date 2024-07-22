mod api;
/**启动程序
*/
mod config;
mod core;
mod model;
mod result;
mod router;
mod utils;
use actix_web::{web, App, HttpResponse, HttpServer};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/admin").configure(router::admin::admin_route))
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/ is here") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
