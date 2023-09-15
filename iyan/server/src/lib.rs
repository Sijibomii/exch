use actix_cors::Cors;
use uuid::Uuid;
use actix_web::{web, App, middleware, HttpResponse, HttpServer};
use core::client::{Client, ClientPayload};
use core::{db::postgres::{PgExecutorAddr}};
use std::{fs};
use config::Config; 
use core::db::postgres;
use rabbitmq::listener::RabbitClient;
use rabbitmq::sender::RabbitSender;
use std::fs::File;
use std::io::Write;
use actix::prelude::*; 
use deadpool_lapin::{Manager, Pool};
use lapin::{ConnectionProperties};

mod controllers;
mod services;
mod state;
mod auth;


async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}



pub async fn run(
    config: Config
) -> std::io::Result<()> {
    
    let postgres_url = config.postgres.clone();
    let pg_pool = postgres::init_pool(&postgres_url);
    let postgres = postgres::PgExecutor::start(postgres::PgExecutor(pg_pool.clone()));
    
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
    let _ = RabbitClient::start(RabbitClient::new(pool.clone(), "balance".to_string(), p.clone()));
   
    // start sender
    
    let rabbit_sender = RabbitSender::start(RabbitSender::new(p0.clone(), "authentication".to_string()));
    
    let balance_sender = RabbitSender::start(RabbitSender::new(p1.clone(), "balance".to_string()));

    let app_state = state::AppState{
        postgres: postgres.clone(),
        config: config.server.clone(),
        jwt_public: fs::read(config.server.public_key.clone())
            .expect("failed to open the public key file"),
        jwt_private: fs::read(config.server.private_key.clone())
            .expect("failed to open the private key file"),
        rabbit_sender: rabbit_sender.clone(),
        balance_sender: balance_sender.clone()
    };
    // write the tokens to file
    setup(&app_state.postgres).await;

    // check for client count. if not insert one.
    let _: Client = match services::client::get_client_count(&postgres).await {
        Ok(client) => {
            client
        }
        Err(_) => {
            // could not find the client
            let mut payload = ClientPayload::new();
            payload.next_id = Some(0);
            payload.id = Some(Uuid::new_v4());
            match services::client::insert(payload, &postgres).await {
                Ok(ctl) => {
                    ctl
                }
                Err(_) => panic!("could not insert new client count")
            }
        }
    };

    let _ = config.server.host.clone();
    let _ = config.server.port.clone();
    
    HttpServer::new(move || {
        App::new()
        // .app_data(app_state.clone())
        .wrap(middleware::Logger::default())
        .wrap(
            Cors::default()
                    .allow_any_origin()   
                    .send_wildcard()      
                    .max_age(3600),
        )
        .service(
            web::scope("/api/v1")
                    .route("/hello", web::get().to(index))
                    .route("/login", web::post().to(controllers::auth::authentication))
                    .route("/register", web::post().to(controllers::auth::registration))
                    .route("/user/me", web::get().to(controllers::auth::profile))
                    .route("/user/{id}", web::delete().to(controllers::auth::delete))
                    .route("/token", web::post().to(controllers::token::create_token))
                    .route("/token", web::get().to(controllers::token::get_all_tokens))
                    .route("/token/{id}", web::get().to(controllers::token::get_token))
                    .route("/token/{id}", web::delete().to(controllers::token::delete_token))
                    .route("/token/trade", web::post().to(controllers::token::begin_trading_token))
                    .route("/token/halt", web::post().to(controllers::token::halt_trading_token))
                    .route("/wallet/", web::post().to(controllers::wallet::create_wallet))
                    .route("/wallet/", web::get().to(controllers::wallet::get_user_wallets))
                    .route("/wallet/{id}", web::get().to(controllers::wallet::get_wallet))
                    .route("/wallet/{id}", web::delete().to(controllers::wallet::delete_wallet))
                    .route("/wallet/{id}/fund", web::post().to(controllers::wallet::fund_wallet))
        )      
    })
    .bind(format!("{}:{}", "localhost", 4001))
    .expect(&format!("can not bind {}:{}", "localhost", 4001))
    .run()
    .await

    
}

pub async fn setup(postgres: &PgExecutorAddr) {
    let file_path = "/tokens/ticker.txt";
    let res = services::tokens::get_all_tokens(100, 0, postgres).await;
    match res {
        Ok(tokens) => {
            let json_data = serde_json::to_string(&tokens).unwrap();
            let mut file = File::create(file_path).unwrap();
            file.write_all(json_data.as_bytes()).unwrap();
        }
         _ => {}
    }
    
}