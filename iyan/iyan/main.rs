extern crate actix;
extern crate server; 
extern crate core;
extern crate config;
use server::run;
use actix::prelude::*; 
use std::{env};
use logger::{init_elasticsearch_logger};
// use core::db::postgres::{self};



#[actix_web::main]
async fn main() {
    env::set_var(
        "RUST_LOG",
        "info,error,debug,actix_web=info,tokio_reactor=info",
    );

    init_elasticsearch_logger("http://logstash:5000").unwrap();

    log::info!("This is an info log message.");
    log::error!("This is an error log message.");


    log::info!("reading iyan.toml");

    let config: config::Config  = config::Config{
        postgres: "postgresql://exch:exch@localhost:5433/exch".to_string(),
        server: config::ServerConfig{
            host: "localhost".to_string(),
            port: 4001,
            mail_sender:"noreply@example.com".to_string(),
            web_client_url:"https://example.com".to_string(),
            public_key:"/keys/private_key.pem".to_string(),
            private_key:"/keys/public_key.pem".to_string()
        },
        smtp: config::SmtpConfig { host: "smtp.example.com".to_string(), port: 587, user: "smtpuser".to_string(), pass: "smtppassword".to_string() }
    };

    let system = System::new();
    log::info!("Running server");
    let _ = run(config).await;
    log::info!("Server up and running");
    let _ = system.run();

}

