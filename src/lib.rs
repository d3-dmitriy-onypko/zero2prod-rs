use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check(_: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}
async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    println!("{}{}", form.email, form.name);
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
