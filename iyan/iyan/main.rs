extern crate actix;
extern crate server; 
extern crate core;
extern crate config;
use server::run;
use std::io::Read;
use actix::prelude::*; 
use std::{env, fs::File};

use std::sync::Arc;
use logger::{init_elasticsearch_logger};

use core::db::postgres;
use rabbitmq::listener::RabbitClient;
use rabbitmq::sender::RabbitSender;
use deadpool_lapin::{Manager, Pool, PoolError};
use lapin::{options::*, types::FieldTable, BasicProperties, ConnectionProperties};
use tokio_amqp::*;

#[tokio::main]
async fn main() {
    env::set_var(
        "RUST_LOG",
        "info,error,debug,actix_web=info,tokio_reactor=info",
    );

    init_elasticsearch_logger("http://logstash:5000").unwrap();

    log::info!("This is an info log message.");
    log::error!("This is an error log message.");

    // empty strings
    let mut settings = String::new();

    log::info!("reading iyan.toml");

    File::open(
        format!("./iyan/iyan.toml").as_str(),
    ) .and_then(|mut f| f.read_to_string(&mut settings)).unwrap();

    let config: config::Config = toml::from_str(&settings).unwrap();

    let system = System::new();

    let postgres_url = config.postgres.clone();
    let pg_pool = postgres::init_pool(&postgres_url);
    let postgres = SyncArbiter::start(4, move || postgres::PgExecutor(pg_pool.clone()));

    let p = postgres.clone();

    let addr =
        std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://rmq:rmq@127.0.0.1:5672/%2f".into());
    let manager = Manager::new(addr, ConnectionProperties::default().with_tokio());
    let pool: Pool = deadpool::managed::Pool::builder(manager)
        .max_size(10)
        .build()
        .expect("can create pool");

    let p0 = pool.clone();

    // start listener
    let rabbit_listener = SyncArbiter::start(1, move || RabbitClient::new(pool.clone(), "balance".to_string(), p.clone()));
   
    // start sender
    let rabbit_sender = SyncArbiter::start(1, move || RabbitSender::new(p0.clone(), "authentication".to_string()));
    
    log::info!("Running server");
    run(postgres, rabbit_sender, config);
    log::info!("Server up and running");
    let _ = system.run();

}
