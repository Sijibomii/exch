use actix_cors::Cors;
// use actix::prelude::*;
use actix_web::{web, App, middleware, HttpResponse, HttpServer};

use std::{fs};
use config::Config; 
use core::db::postgres;


mod state;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world!")
}

pub async fn run(postgres: postgres::PgExecutorAddr, config: Config) -> std::io::Result<()> {

    let app_state = state::AppState{
        postgres: postgres.clone(),
        config: config.server.clone(),
        jwt_public: fs::read(config.server.public_key.clone())
            .expect("failed to open the public key file"),
        jwt_private: fs::read(config.server.private_key.clone())
            .expect("failed to open the private key file"),
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
                    .route("/login", web::post().to(index))
        )
    })
    .bind(format!("{}:{}", host, port))
    .expect(&format!("can not bind {}:{}", host, port))
    .run()
    .await
}