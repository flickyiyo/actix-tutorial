use actix_web::{self, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use repositories::{MemoryRepository, Repository};
use std::str::FromStr;
mod handlers;
mod models;
mod repositories;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("mundo");
    format!("Hola {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=debug");
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("127.0.0.1:{}", port);
    println!("Starting our server");
    HttpServer::new(|| {
        println!("Starting our thread");
        App::new()
            .service(web::resource("/user/{userId}").route(web::get().to(get_user)))
            .route("/health", web::get().to(|| HttpResponse::Ok()))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .workers(2)
    .run()
    .await
}

// fn get_user(req: HttpRequest) -> HttpResponse {
//     if let Some(user_id) = req.match_info().get("userId") {
//         let repo = MemoryRepository::default();
//         match uuid::Uuid::from_str(user_id) {
//             Ok(parsed_user_id) => {
//                 match repo.get_users(parsed_user_id) {
//                     Ok(user) => HttpResponse::Ok().json(user),
//                     Err(err) => HttpResponse::InternalServerError().body(err.to_string())
//                 }
//             },
//             Err(_) => HttpResponse::NotFound().body("Not found")
//         }
//     } else {
//         HttpResponse::BadRequest().body("User ID Not Found")
//     }
// }

fn get_user(user_id: web::Path<String>) -> HttpResponse {
    if let Ok(parsed_user_id) =uuid::Uuid::from_str(&user_id) {
        let repo = MemoryRepository::default();
        match repo.get_users(&parsed_user_id) {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::NotFound().body("Not found")
        }
    } else {
        HttpResponse::NotFound().body("Invalid uuid")
    }
}