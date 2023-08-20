use actix_web::{web, Responder};
use crate::AppState; // Import your AppState type

#[derive(Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}


async fn authentication(data: web::Data<AppState>, user_data: web::Json<LoginParams>) -> impl Responder {
    // Access state fields using data.field_name
    // For example: data.postgres, data.mailer, etc.

    // Access the deserialized JSON data from user_data
    println!("Received username: {}", user_data.username);
    println!("Received password: {}", user_data.password);

    // ... Your handler logic ...

    HttpResponse::Ok().json("Data received successfully")
}