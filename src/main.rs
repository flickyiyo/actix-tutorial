use actix_web::{self, App, HttpRequest, HttpResponse, HttpServer, Responder, Result, web::{self, Data}};
use repositories::{MemoryRepository, Repository, RepositoryInjector};
use core::panic;
use std::{str::FromStr, sync::Arc};
mod handlers;
mod models;
mod repositories;
mod v1;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("mundo");
    format!("Hola {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    dotenv::dotenv().ok();
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("127.0.0.1:{}", port);
    println!("Starting our server");
    let repo = MemoryRepository::default();
    let repo = RepositoryInjector::new_shared(repo);
    HttpServer::new(move || {
        println!("Starting our thread");
        App::new()
            .data(repo.clone())
            .route("/", web::get().to(hola_rust))
            .service(web::resource("/user/{userId}").route(web::get().to(get_user)))
            .route("/health", web::get().to(|| HttpResponse::Ok()))
            .route("/{name}", web::get().to(greet))
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!("Could not initialize app on port {}: {:?}", port, err)
    })
    .workers(2)
    .run()
    .await
}

async fn hola_rust(req: HttpRequest, repo: Data<Arc<dyn Repository>>) -> impl Responder {
    format!("Hola Rust")
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

fn get_user(user_id: web::Path<String>, repo: Data<Arc<RepositoryInjector>>) -> HttpResponse {
    if let Ok(parsed_user_id) =uuid::Uuid::from_str(&user_id) {
        match repo.get_users(&parsed_user_id) {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(_) => HttpResponse::NotFound().body("Not found")
        }
    } else {
        HttpResponse::NotFound().body("Invalid uuid")
    }
}