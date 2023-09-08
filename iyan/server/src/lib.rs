use actix_cors::Cors;
use uuid::Uuid;
use actix_web::{web, App, middleware, HttpResponse, HttpServer};
use core::client::{Client, ClientPayload};

use std::{fs};
use config::Config; 
use core::db::postgres;
use rabbitmq::sender::RabbitSenderAddr;

mod controllers;
mod services;
mod state;
mod auth;

// use services::{ errors::Error };

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

pub async fn run(postgres: postgres::PgExecutorAddr, rabbit_sender: RabbitSenderAddr,config: Config) -> std::io::Result<()> {


    let app_state = state::AppState{
        postgres: postgres.clone(),
        config: config.server.clone(),
        jwt_public: fs::read(config.server.public_key.clone())
            .expect("failed to open the public key file"),
        jwt_private: fs::read(config.server.private_key.clone())
            .expect("failed to open the private key file"),
        rabbit_sender: rabbit_sender.clone()
    };

    // check for client count. if not insert one.
    let ctl: Client = match services::client::get_client_count(&postgres).await {
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

    let host = config.server.host.clone();
    let port = config.server.port.clone();

    HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
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
    .bind(format!("{}:{}", host, port))
    .expect(&format!("can not bind {}:{}", host, port))
    .run()
    .await
}