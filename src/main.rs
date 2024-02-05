
mod models;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde_json::json; // Add this import
use models::User; // Add this import
use models::Response;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


async fn response_example() -> impl Responder {
    HttpResponse::Ok().json(Response {
        message: "Hello, World!".to_string()
    })
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

async fn json_request (user: web::Json<User>) -> impl Responder{
   
   if user.name == "Oguzhan" {

    HttpResponse::Ok().json(json!({
        "message": "Name is valid"
    }))

    
   }
    else {
      HttpResponse::BadRequest().json(json!({
          "error": "Name is not valid"
     }))
    }

}


#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/json", web::get().to(json_response))
            .route("/json", web::post().to(json_request))
            .route("/response", web::get().to(response_example))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
