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
use rabbitmq::sender::{self, Sender, TokenData};
use core::client::Client;
use log::{debug};
#[derive(Deserialize)]
pub struct CreateTokenParams {
    pub ticker: String,
    pub supply: i64,
}

pub async fn create_token(
    data: web::Json<CreateTokenParams>,
    state: web::Data<AppState>,
    user: AuthUser
) -> Result<Json<Value>, Error>  {
    let params = data.into_inner();

    let mut payload = TokenPayload::new();

    let ctl: Result<Client, Error> = match services::client::get_client_count(&state.postgres).await {
        Ok(client) => {
            Ok(client)
        }
        Err(err) => {
            Err(err.into())
        }
    };
    let client_ = ctl.unwrap();
    // check for the last ticker_id
    payload.is_trading = Some(true);
    payload.user_id = Some(user.id);
    payload.supply = Some(params.supply);
    payload.ticker = Some(params.ticker);
    payload.ticker_id = Some(client_.next_ticker_id);
    

    let res = services::tokens::create(
        payload,
        &state.postgres
    ).await;

    match res {
        Ok(token) => {
            let pay = TokenData { ticker_id: token.ticker_id};
            // chnage migrations
            match Sender::publish_new_token(pay, &state.rabbit_sender).await {
                Ok(res) => {
                    match sender::publish(&res.channel, res.queue, &res.data).await {
                        Ok(_) => {
                            debug!("controller: published successfully");
                            return Ok(Json(json!({ "token" : token })));
                        }

                        Err(_) => {
                            debug!("controller: could not successfully publish");
                            panic!("could not successfully publish")
                        }
                    }
                }
                Err(_) => {
                    debug!("controller: could not send publish message");
                    panic!("could not send publish message")
                }
            }
            
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    };
}

// get token
pub async fn get_token(
    state: web::Data<AppState>,
    path: web::Path<Uuid>, 
    user: AuthUser,
) -> Result<Json<Value>, Error> {
    let id = path.into_inner();
    if user.id.is_nil() {
        return Err(Error::UnAuthorizedRequestAccount);
    }
    let res = services::tokens::get(id, &state.postgres).await;
    match res {
        Ok(token) => {
            return Ok(Json(json!({ "token": token })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    }
}

const LIMIT: i64 = 15;
const OFFSET: i64 = 0;

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// get all tokens
pub async fn get_all_tokens(
    data: web::Json<ListParams>,
    state: web::Data<AppState>
) -> Result<Json<Value>, Error> {
    let mut limit = LIMIT;
    let mut offset = OFFSET;

    if let Some(_limit) = data.limit { 
        if _limit < LIMIT {
            limit = _limit;
        }
    };

    if let Some(_offset) = data.offset {
        offset = _offset;
    };

    let res = services::tokens::get_all_tokens(limit, offset, &state.postgres).await;
    match res {
        Ok(token) => {
            return Ok(Json(json!({ "token": token })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    }
}

#[derive(Deserialize)]
pub struct StartTradingTokenParams {
    pub id: Uuid
}

// start token trading
pub async fn begin_trading_token(
    data: web::Json<StartTradingTokenParams>,
    state: web::Data<AppState>,
    user: AuthUser
) -> Result<Json<Value>, Error> {

    if user.id.is_nil() {
        return Err(Error::UnAuthorizedRequestAccount);
    }
    let res = services::tokens::start_trading_token(data.id, &state.postgres).await;
    match res {
        Ok(token) => {
            return Ok(Json(json!({ "token": token })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    }
}

// turn off trading
pub async fn halt_trading_token(
    data: web::Json<StartTradingTokenParams>,
    state: web::Data<AppState>,
    user: AuthUser
) -> Result<Json<Value>, Error> {

    if user.id.is_nil() {
        return Err(Error::UnAuthorizedRequestAccount);
    }
    let res = services::tokens::halt_trading_token(data.id, &state.postgres).await;
    match res {
        Ok(token) => {
            return Ok(Json(json!({ "token": token })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    }
}


// delete token
pub async fn delete_token(
    data: web::Json<StartTradingTokenParams>,
    state: web::Data<AppState>,
    user: AuthUser
) -> Result<Json<Value>, Error> {
    if user.id.is_nil() {
        return Err(Error::UnAuthorizedRequestAccount);
    }

    let res = services::tokens::get(data.id, &state.postgres).await;

    match res {
        Ok(token) => {
            if user.id != token.user_id {
                return Err(Error::UnAuthorizedRequestAccount);
            }

            match services::tokens::delete(data.id, &state.postgres).await {
                Ok(u) => {
                    return Ok(Json(json!({ "token": u })))
                }
                Err(error) => {
                    return Err(Error::from(error))
                }

            }
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    }
}