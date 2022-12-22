use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    println!("{}{}", form.email, form.name);
    HttpResponse::Ok()
}
