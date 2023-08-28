use uuid::Uuid;
use super::super::schema::{wallet};
use super::super::db::{
    postgres::PgExecutorAddr,
   wallet::{Insert, Update, FindAllWalletsByUser, FindWalletById, DeleteWallet}
};
use chrono::{prelude::*};
use super::super::models::errors::Error;
use diesel::prelude::*;
use serde_json::Value;

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = wallet)]
pub struct WalletPayload {
    pub id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub balance: Option<i64>,
    pub last_activity_time: Option<DateTime<Utc>>,
}   

impl WalletPayload {
    pub fn new() -> Self {
        WalletPayload {
            id: None,
            user_id: None,
            balance: None,
            last_activity_time: None
        }
    }
}

impl From<Wallet> for WalletPayload {
    fn from(wallet: Wallet) -> Self {
        WalletPayload {
            id: Some(wallet.id),
            user_id: Some(wallet.user_id),
            balance: Some(wallet.balance),
            last_activity_time: Some(if wallet.last_activity_time.is_some() { wallet.last_activity_time.unwrap() } else { Utc::now() } ) 
        }
    }
}

// Associations,
#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = wallet)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: i64,
    pub last_activity_time: Option<DateTime<Utc>>,
}

impl Wallet {

    pub async fn insert(
        payload: WalletPayload,
        postgres: &PgExecutorAddr, 
    ) -> Result<Wallet, Error> {
        (*postgres)
        .send(Insert(payload))
        .await
        .map_err(Error::from)
        .and_then(|res| {
            res.map_err(|e| Error::from(e))
        })
    }


    pub async fn update(
        id: Uuid,
        payload: WalletPayload,
        postgres: &PgExecutorAddr,
    ) -> Result<Wallet, Error> {

        (*postgres)
            .send(Update { id, payload })
            .await
            .map_err(Error::from)
            .and_then(|res| {
                res.map_err(|e| Error::from(e))
            })
    }

    // find wallet by user
    pub async fn find_all_wallets_by_user(
        id: Uuid,
        limit: i64,
        offset: i64,
        postgres: &PgExecutorAddr
    ) -> Result<Vec<Wallet>, Error>  {
        (*postgres)
            .send(FindAllWalletsByUser{ id, offset, limit })
            .await
            .map_err(Error::from)
            .and_then(|res| {
                res.map_err(|e| Error::from(e))
            })
    }

    pub async fn find_by_id(
        id: Uuid,
        postgres: &PgExecutorAddr,
    ) -> Result<Wallet, Error> {
        (*postgres)
        .send(FindWalletById(id))
        .await
        .map_err(Error::from)
        .and_then(|res| {
            res.map_err(|e| Error::from(e))
        })
    }

    pub async fn delete(
        id: Uuid,
        postgres: &PgExecutorAddr,
    ) -> Result<usize, Error>  {
        (*postgres)
        .send(DeleteWallet(id))
        .await
        .map_err(Error::from)
        .and_then(|res| {
            res.map_err(|e| Error::from(e))
        })
    }

    pub fn export(&self) -> Value {
        json!({
            "id": self.id,
            "user": self.user_id,
            "balance": self.balance
        })
    }

}