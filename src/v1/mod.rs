mod users;
use actix_web::{web::{ServiceConfig, self}, HttpResponse};
pub use users::{service as user_service};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/v1")
        .route("/health", web::get().to(health_check))
        .route("/ther", web::get().to(health_check))
    );

}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .finish()
}