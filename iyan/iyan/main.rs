extern crate actix;
extern crate server; 
extern crate core;
extern crate config;
use server::run;
use std::io::Read;
use actix::prelude::*; 
use std::{env, fs::File};

use logger::{init_elasticsearch_logger};

use core::db::postgres;
fn main() {
    env::set_var(
        "RUST_LOG",
        "info,error,debug,actix_web=info,tokio_reactor=info",
    );
    env_logger::init();

    init_elasticsearch_logger("http://logstash:5000");

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
    log::info!("Running server");
    server::run(postgres, config);
    log::info!("Server up and running");
    let _ = system.run();

    // export LDFLAGS="-L/opt/homebrew/opt/postgresql@15/lib"
//   export CPPFLAGS="-I/opt/homebrew/opt/postgresql@15/include"
// Or, if you don't want/need a background service you can just run:
//   LC_ALL="C" /opt/homebrew/opt/postgresql@15/bin/postgres -D /opt/homebrew/var/postgresql@15

// /opt/homebrew/Cellar/postgresql@15/15.4  export LIBRARY_PATH=/opt/homebrew/opt/postgresql@15/lib:$LIBRARY_PATH
}
