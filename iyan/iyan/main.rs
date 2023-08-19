extern crate actix;
extern crate server; 
extern crate core;
extern crate config;

use std::io::Read;
use actix::prelude::*; 
use std::{env, fs::File};

use core::db::postgres;
fn main() {
    env::set_var(
        "RUST_LOG",
        "info,error,debug,actix_web=info,tokio_reactor=info",
    );
    env_logger::init();

    // empty strings
    let mut settings = String::new();

    File::open(
        format!("{}/iyan.toml", env!("HOME")).as_str(),
    ) .and_then(|mut f| f.read_to_string(&mut settings)).unwrap();

    let config: config::Config = toml::from_str(&settings).unwrap();

    let system = System::new();

    let postgres_url = config.postgres.clone();
    let pg_pool = postgres::init_pool(&postgres_url);
    let postgres = SyncArbiter::start(4, move || postgres::PgExecutor(pg_pool.clone()));

    server::run(postgres, config);
    let _ = system.run();
}
