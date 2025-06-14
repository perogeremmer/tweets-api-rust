use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
mod controller;
mod helper; // Deklarasi modul helper di root


// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(controller::main_controller::hello) // Registering the hello service from main_controller
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

