extern crate actix;
extern crate server; 
extern crate core;
extern crate config;
use server::run;

use std::{env};

extern crate diesel_migrations;
use diesel::{PgConnection, Connection};
use diesel_migrations::{embed_migrations, MigrationHarness, EmbeddedMigrations};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn run_db_migrations(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
}

#[actix_web::main]
async fn main() {
    env::set_var(
        "RUST_LOG",
        "info,error,debug,actix_web=info,tokio_reactor=info",
    );

    let config: config::Config  = config::Config{
        postgres: "postgresql://exch:exch@postgres:5432/exch".to_string(),
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
    
    let mut connection = PgConnection::establish("postgresql://exch:exch@postgres:5432/exch")
    .expect(&format!("Error connecting to {}", "postgresql://exch:exch@postgres:5432/exch".to_string()));
    
    run_db_migrations(&mut connection);

   
    let _ = run(config).await;

}

