use actix_web::{web, HttpResponse, Responder};

#[allow(unused, unused_imports)]
use sqlx::PgPool;
#[allow(unused, unused_imports)]
use uuid::Uuid;

use crate::domain::{new_subscriber::NewSubscriber, subscriber_name::SubscriberName, subsrciber_email::SubscriberEmail};

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
    let Ok(name) = SubscriberName::parse(s.name) else { 
        return HttpResponse::BadRequest(); 
    };

    let Ok(email) = SubscriberEmail::parse(s.email) else { 
        return HttpResponse::BadRequest(); 
    };

    let subscriber = NewSubscriber {
        name,
        email
    };

    match insert_subscriber(&pool, &subscriber).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
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
        new_subscriber.email.as_ref(),
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
