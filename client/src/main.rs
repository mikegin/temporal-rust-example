use actix_web::{get, web, App, HttpServer, Responder};

mod client;

use client::{execute_workflow};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/execute")]
async fn execute() -> impl Responder {
    execute_workflow().await;
    format!("Executed!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(execute))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}