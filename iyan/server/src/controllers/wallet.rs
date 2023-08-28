use actix_web::{web};
use actix_web::web::Json;
use super::super::state::AppState; 
use chrono::{prelude::*};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use super::super::services::{ self, errors::Error };
use core::wallet::WalletPayload;
use super::super::auth::AuthUser;
use uuid::Uuid;


// create a wallet
pub async fn create_wallet(
    state: web::Data<AppState>,
    user: AuthUser
) -> Result<Json<Value>, Error>  {
    let mut payload = WalletPayload::new();

    payload.user_id = Some(user.id);
    payload.balance = Some(0);
    payload.last_activity_time = Some(Utc::now());


    let res = services::wallet::create(
        payload,
        &state.postgres
    ).await;

    match res {
        Ok(wallet) => {
            return Ok(Json(json!({ "wallet" : wallet })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    };
}


#[derive(Deserialize)]
pub struct FundWalletParams {
    pub deposit: i64,
    pub id: Uuid
}
// fund wallet
pub async fn fund_wallet(
    data: web::Json<FundWalletParams>,
    state: web::Data<AppState>,
    user: AuthUser
) -> Result<Json<Value>, Error> {
     
    match services::wallet::get(data.id, &state.postgres).await {
        Ok(wallet) => {
            if user.id != wallet.user_id {
                return Err(Error::UnAuthorizedRequestAccount);
            }
            match services::wallet::fund_wallet(wallet.id, data.deposit, &state.postgres).await {
                Ok(new_wallet) => {
                    return Ok(Json(json!({ "wallet" : new_wallet })))
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

const LIMIT: i64 = 15;
const OFFSET: i64 = 0;

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// get wallets by user
pub async fn get_user_wallets(
    data: web::Json<ListParams>,
    state: web::Data<AppState>,
    user: AuthUser
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
    match services::wallet::all_wallet_by_user(limit, offset, user.id, &state.postgres).await {
        Ok(wallets) => {
            return Ok(Json(json!({ "wallets": wallets })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    }
}

// get a wallet
pub async fn get_wallet(
    state: web::Data<AppState>,
    path: web::Path<Uuid>, 
    user: AuthUser,
) -> Result<Json<Value>, Error> {
    let id = path.into_inner();
    if user.id.is_nil() {
        return Err(Error::UnAuthorizedRequestAccount);
    }
    let res = services::wallet::get(id, &state.postgres).await;
    match res {
        Ok(wallet) => {
            return Ok(Json(json!({ "wallet": wallet })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    }
}
#[derive(Deserialize)]
pub struct DeleteWalletParams {
    pub id: Uuid
}
// delete wallet
pub async fn delete_wallet(
    data: web::Json<DeleteWalletParams>,
    state: web::Data<AppState>,
    user: AuthUser
) -> Result<Json<Value>, Error> {
    if user.id.is_nil() {
        return Err(Error::UnAuthorizedRequestAccount);
    }

    let res = services::wallet::get(data.id, &state.postgres).await;

    match res {
        Ok(wallet) => {
            if user.id != wallet.user_id {
                return Err(Error::UnAuthorizedRequestAccount);
            }

            match services::wallet::delete(data.id, &state.postgres).await {
                Ok(u) => {
                    return Ok(Json(json!({ "wallet": u })))
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