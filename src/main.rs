use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    let port = listener.local_addr().unwrap().port();
    println!("App bound to address: http://127.0.0.1:{}", port);
    run(listener, connection)?.await
}
