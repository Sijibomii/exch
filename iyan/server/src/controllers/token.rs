use actix_web::{web};
use actix_web::web::Json;
use super::super::state::AppState; 
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use super::super::services::{ self, errors::Error };
use core::token::TokenPayload;
use super::super::auth::AuthUser;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTokenParams {
    pub ticker: String,
    pub supply: i64,
}

pub async fn createToken(
    data: web::Json<CreateTokenParams>,
    state: web::Data<AppState>,
    user: AuthUser
) -> Result<Json<Value>, Error>  {
    let params = data.into_inner();

    let mut payload = TokenPayload::new();

    payload.is_trading = Some(false);
    payload.user_id = Some(user.id);
    payload.supply = Some(params.supply);
    payload.ticker = Some(params.ticker);

    let res = services::tokens::create(
        payload,
        &state.postgres
    ).await;

    match res {
        Ok(token) => {
            return Ok(Json(json!({ "token" : token })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    };
}