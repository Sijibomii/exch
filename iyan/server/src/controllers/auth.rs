use actix_web::{web, Responder, HttpResponse};
use super::super::state::AppState; // Import your AppState type
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}
