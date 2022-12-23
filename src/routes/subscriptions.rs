use actix_web::{web, HttpResponse, Responder};
use sqlx::{types::chrono::Utc, PgConnection, PgPool};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions(id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await;
    HttpResponse::Ok()
}
