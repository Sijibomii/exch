use uuid::Uuid;
use super::super::schema::{wallet};
use super::super::db::{
    postgres::PgExecutorAddr,
   
};
use chrono::{prelude::*, Duration};
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
    
}