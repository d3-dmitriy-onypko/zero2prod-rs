use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod_rs::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let subscriber = get_subscriber("zero2prod_rs".into(), "debug".into(), std::io::stdout);
    init_subscriber(subscriber);
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("failed");
    let pool = PgPool::connect_lazy_with(configuration.database.with_db());
    run(listener, pool)?.await
}
