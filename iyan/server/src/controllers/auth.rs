use actix_web::{web};
use actix_web::web::Json;
use super::super::state::AppState; 
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use super::super::services::{ self, errors::Error };
use core::user::{UserPayload};
use core::client::Client;
use super::super::auth::AuthUser;
use uuid::Uuid;

use rabbitmq::sender::{Sender, Data, Wallet, DataNoWallet};

#[derive(Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

const LIMIT: i64 = 1;
const OFFSET: i64 = 0;

// take care of user login without wallet
// login 
pub async fn authentication(
    data: web::Json<LoginParams>,
    state: web::Data<AppState>
) -> Result<Json<Value>, Error>  {
    let params = data.into_inner();

    let res = services::users::authenticate(
        params.email,
        params.password,
        &state.postgres,
        state.jwt_private.clone(),
    ).await;

    match res {
        Ok((token, user)) => {
            let wallets = services::wallet::all_wallet_by_user(LIMIT, OFFSET, user.id, &state.postgres).await;
            match wallets {
                Ok(wallets) => {
                    match wallets.first() {

                        Some(wallet) => {
                            let u = user.clone();
                            let payload = Data{
                                user_id: u.id,
                                email: u.email,
                                trading_client_id: u.trading_client_id,
                                last_order_number: u.last_order_id,
                                last_seq_num: u.last_seq_num,
                                wallet: Wallet{
                                    id: wallet.id,
                                    balance: wallet.balance
                                }
                            };
                            Sender::publish_login(payload, &state.rabbit_sender).await;
                            return Ok(Json(json!({ "token": token, "user": user.export() })));
                        }
                        None => {
                            // still publish but set wallet to nil -> 
                            let u = user.clone();
                            let payload = DataNoWallet{
                                user_id: u.id,
                                email: u.email,
                                trading_client_id: u.trading_client_id,
                                last_order_number: u.last_order_id,
                                last_seq_num: u.last_seq_num,
                            };
                            Sender::publish_login_no_wallet(payload, &state.rabbit_sender).await;
                            return Ok(Json(json!({ "token": token, "user": user.export() })));
                        }
                    }

                }
                Err(error) => {
                    return Err(Error::from(error))
                }
            }
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    };
}



#[derive(Debug, Deserialize)]
pub struct RegistrationParams {
    pub email: String, 
    pub password: String,
}
// registration
pub async fn registration(
    data: web::Json<RegistrationParams>,
    state: web::Data<AppState>
) ->  Result<Json<Value>, Error> {
    
    let params = data.into_inner();

    if params.email.len() == 0 {
        return Err(Error::BadRequest("email is empty"));
    }

    if params.password.len() == 0 {
        return Err(Error::BadRequest("password is empty"));
    }

    // get the latest client_id
    let ctl: Result<Client, Error> = match services::client::get_client_count(&state.postgres).await {
        Ok(client) => {
            Ok(client)
        }
        Err(err) => {
            Err(err.into())
        }
    };

    let client_ = ctl.unwrap();
    // let curr_client_id = services

    let mut payload = UserPayload::new();
    payload.email = Some(params.email);
    payload.password = Some(params.password);
    payload.last_seq_num = Some(0);
    payload.last_order_id = Some(0);
    payload.trading_client_id = Some(client_.next_id);

    let res = services::users::register(
        payload,
        &state.postgres
    ).await;

    services::client::increase_client_count(&state.postgres).await;

    match res {
        Ok(user) => {
            return Ok(Json(json!({ "user": user.export() })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    }
}

//  get the entire user
pub async fn profile( 
    state: web::Data<AppState>,
    user: AuthUser
) -> Result<Json<Value>, Error> {
    let res = services::users::get(user.id, &state.postgres).await;

    match res {
        Ok(user) => {
            return Ok(Json(json!({ "user": user.export() })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    }
}

// this is how to get path parms in rust. the uuid is passed in
pub async fn delete(
    state: web::Data<AppState>, // Access the app state
    path: web::Path<Uuid>, // Extract dynamic path parameter
    user: AuthUser,
) -> Result<Json<Value>, Error> {
    let id = path.into_inner();
    if id != user.id {
        return Err(Error::UnAuthorizedRequestAccount);
    }
    let res = services::users::delete(user.id, &state.postgres).await;
    match res {
        Ok(us) => {
            return Ok(Json(json!({ "deleted": us })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    }
}

