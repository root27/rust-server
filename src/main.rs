
mod models;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde_json::json; // Add this import
use models::User; // Add this import


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}



async fn json_response () -> impl Responder {
    HttpResponse::Ok().json(json!({
        "name": "John Doe",
        "age": 43,
        "address": {
            "street": "10 Downing Street",
            "city": "London"
        }
    }))
}

async fn json_request (user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user.0)
}


#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/json", web::get().to(json_response))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
