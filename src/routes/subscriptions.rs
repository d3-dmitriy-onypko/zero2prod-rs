use actix_web::{web, HttpResponse, Responder};

#[allow(unused, unused_imports)]
use sqlx::PgPool;
#[allow(unused, unused_imports)]
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
#[tracing::instrument(
 name = "Adding a new subscriber", skip(form, pool),
 fields(
    subscriber_email = %form.email,
    subscriber_name= %form.name
 )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let s = form.into_inner();
    let subscriber = NewSubscriber {
        name: SubscriberName::parse(s.name),
        email: s.email,
    };

    match insert_subscriber(&pool, &subscriber).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
    .await
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.as_ref(),
        sqlx::types::time::OffsetDateTime::now_utc()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    // Using the `?` operator to return early
    // if the function failed, returning a sqlx::Error // We will talk about error handling in depth later! })?;
    Ok(())
}
