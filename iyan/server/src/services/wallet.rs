use core::{db::postgres::{PgExecutorAddr}, wallet::{ Wallet, WalletPayload}};
use super::errors::Error;
use uuid::Uuid;

// create a wallet
pub async fn create(
    payload: WalletPayload,
    postgres: &PgExecutorAddr,
) -> Result<Wallet, Error> {
    match Wallet::insert(payload, postgres).await {
        Ok(wallet) => {
            Ok(wallet)
        }

        Err(err) => {
            Err(err.into())
        }
    }
}


// fund wallet
pub async fn fund_wallet(
    id: Uuid,
    deposit: i64,
    postgres: &PgExecutorAddr,
) -> Result<Wallet, Error> {
    
    match Wallet::find_by_id(id, postgres).await {
        Ok(wallet) => {
            let mut payload = WalletPayload::from(wallet);
            payload.balance = if payload.balance.is_some() { Some(payload.balance.unwrap() + deposit) } else { Some(deposit) };
            match Wallet::update(id, payload, postgres).await {
                Ok(ret_wallet) => {
                    Ok(ret_wallet)
                }
                Err(err) => {
                    Err(err.into())
                }
            }
        }
        Err(err) => {
            Err(err.into())
        }
    }
} 

// get all wallet by user
pub async fn all_wallet_by_user(
    limit: i64, 
    offset: i64,
    user_id: Uuid,
    postgres: &PgExecutorAddr,
) -> Result<Vec<Wallet>, Error> {
    match Wallet::find_all_wallets_by_user(user_id, limit, offset, postgres).await {
        Ok(wallets) => {
            Ok(wallets)
         }
         Err(err) => {
             Err(err.into())
         }
    }
}

// get a wallet
pub async fn get(id: Uuid, postgres: &PgExecutorAddr) -> Result<Wallet, Error> {
    match Wallet::find_by_id(id, postgres).await {
        Ok(wallet) => {
            Ok(wallet)
         }
         Err(err) => {
             Err(err.into())
         }
    }
}

// delete wallet
pub async fn delete(id: Uuid, postgres: &PgExecutorAddr) -> Result<usize, Error> {
    match Wallet::delete(id, postgres).await {
        Ok(u) => {
            Ok(u)
        }
        Err(err) => {
            Err(err.into())
        }
    }
} 