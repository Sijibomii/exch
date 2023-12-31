use uuid::Uuid;
use super::super::schema::{tokens};
use super::super::db::{
    postgres::PgExecutorAddr,
    token::{Insert, Update, FindTokenById, DeleteToken, FindAllTradedTokens}
};
use super::super::models::errors::Error;
use diesel::prelude::*;
use serde_json::Value;

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = tokens)]
pub struct TokenPayload {
    pub id: Option<Uuid>,
    pub ticker: Option<String>, 
    pub is_trading: Option<bool>,
    pub supply: Option<i64>,
    pub user_id: Option<Uuid>,
    pub ticker_id: Option<i64>
}

impl TokenPayload {
    pub fn new() -> Self {
        TokenPayload {
            id: None,
            ticker: None,
            is_trading: None,
            supply: None,
            user_id: None,
            ticker_id: None
        }
    }
}

impl From<Token> for TokenPayload {
    fn from(token: Token) -> Self {
        TokenPayload {
            id: Some(token.id),
            ticker: Some(token.ticker), 
            is_trading: Some(token.is_trading),
            supply: Some(token.supply),
            user_id: Some(token.user_id),
            ticker_id: Some(token.ticker_id)
        }
    }
}
// Associations,
#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = tokens)]
pub struct Token {
    pub id: Uuid,
    pub ticker: String, 
    pub is_trading: bool,
    pub supply: i64,
    pub user_id: Uuid,
    pub ticker_id: i64
}


impl Token {

    pub async fn insert(
        payload: TokenPayload,
        postgres: &PgExecutorAddr, 
    ) -> Result<Token, Error> {
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
        payload: TokenPayload,
        postgres: &PgExecutorAddr,
    ) -> Result<Token, Error> {

        (*postgres)
            .send(Update { id, payload })
            .await
            .map_err(Error::from)
            .and_then(|res| {
                res.map_err(|e| Error::from(e))
            })
    }

    pub async fn find_all_traded_tokens(
        limit: i64,
        offset: i64,
        postgres: &PgExecutorAddr
    ) -> Result<Vec<Token>, Error>  {
        (*postgres)
            .send(FindAllTradedTokens{ offset, limit })
            .await
            .map_err(Error::from)
            .and_then(|res| {
                res.map_err(|e| Error::from(e))
            })
    }

    pub async fn find_by_id(
        id: Uuid,
        postgres: &PgExecutorAddr,
    ) -> Result<Token, Error> {
        (*postgres)
        .send(FindTokenById(id))
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
        .send(DeleteToken(id))
        .await
        .map_err(Error::from)
        .and_then(|res| {
            res.map_err(|e| Error::from(e))
        })
    }

    pub fn export(&self) -> Value {
        json!({
            "id": self.id,
            "ticker": self.ticker,
            "supply": self.supply
        })
    }

}
