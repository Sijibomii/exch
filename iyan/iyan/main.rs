extern crate actix;
extern crate server; 
extern crate core;
extern crate config;
use server::run;
use actix::prelude::*; 
use std::{env};

use logger::{init_elasticsearch_logger};

use core::db::postgres;
use rabbitmq::listener::RabbitClient;
use rabbitmq::sender::RabbitSender;
use deadpool_lapin::{Manager, Pool};
use lapin::{ConnectionProperties};


fn main() {
    env::set_var(
        "RUST_LOG",
        "info,error,debug,actix_web=info,tokio_reactor=info",
    );

    init_elasticsearch_logger("http://logstash:5000").unwrap();

    log::info!("This is an info log message.");
    log::error!("This is an error log message.");

    let config: config::Config  = config::Config{
        postgres: "postgres://postgres:postgres@localhost:5432/exch".to_string(),
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

    let postgres_url = config.postgres.clone();
    let pg_pool = postgres::init_pool(&postgres_url);
    let postgres = SyncArbiter::start(4, move || postgres::PgExecutor(pg_pool.clone()));

    let p = postgres.clone();

    let addr =
        std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://guest:guest@rabbitmq:5672/%2f".into());
    let manager = Manager::new(addr, ConnectionProperties::default());
    let pool: Pool = deadpool::managed::Pool::builder(manager)
        .max_size(10)
        .build()
        .expect("can create pool");

    let p0 = pool.clone();
    let p1 = pool.clone();
    // start listener
    let _ = SyncArbiter::start(1, move || RabbitClient::new(pool.clone(), "balance".to_string(), p.clone()));
   
    // start sender
    let rabbit_sender = SyncArbiter::start(1, move || RabbitSender::new(p0.clone(), "authentication".to_string()));
    
    let balance_sender = SyncArbiter::start(1, move || RabbitSender::new(p1.clone(), "balance".to_string()));
    log::info!("Running server");
    let _ = run(postgres, rabbit_sender, balance_sender, config);
    log::info!("Server up and running");
    let _ = system.run();

}
