use std::net::TcpListener;

use sqlx::{Connection, PgPool};
use zero2prod_rs::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("failed");
    let pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect");
    run(listener, pool)?.await
}
