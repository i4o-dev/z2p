#![allow(dead_code, unused_variables)] // TODO: remove when done

use sqlx::PgPool;
use std::net::TcpListener;
use z2p::{config::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)?;
    println!("Listening for connections at {address}");
    run(listener, connection_pool)?.await
}
