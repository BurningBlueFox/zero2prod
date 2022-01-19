use std::net::TcpListener;
use zero2prod::{startup::run, configuration::{get_configuration, self}};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("App bound to address: http://127.0.0.1:{}", port);
    run(listener)?.await
}
