
mod models;
mod database;

use actix_web::{middleware::Logger,web, App, HttpServer, Responder, HttpResponse};
// use serde_json::json; // Add this import
use models::User; // Add this import
use models::Response;


use database::mongo_repo::MongoRepo; // Add this import




async fn create_user (user: web::Json<User>, db: web::Data<MongoRepo>) -> impl Responder {
    let result = db.create_user(user.into_inner()).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(Response { message: "User created successfully".to_string() }),
        Err(_) => HttpResponse::InternalServerError().json(Response { message: "Failed to create user".to_string() })
    }
}


async fn get_user(db: web::Data<MongoRepo>, name: web::Path<String>) -> impl Responder {
    let result = db.get_user(name.into_inner()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().json(Response { message: "Failed to get user".to_string() })
    }
}





#[actix_web::main]

async fn main() -> std::io::Result<()> {


    let db = MongoRepo::init().await;

    let app_data = web::Data::new(db);
  

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::new(""))
            .route("/create_user", web::post().to(create_user))
            .route("/get_user/{name}", web::get().to(get_user))
           
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
