use uuid::Uuid;
use super::super::schema::{tokens, users};
use super::user::User;
use super::super::db::{
    postgres::PgExecutorAddr,
    token::{Insert}
};
use super::super::models::errors::Error;


#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = tokens)]
pub struct TokenPayload {
    pub id: Option<Uuid>,
    pub ticker: Option<String>, 
    pub user_id: Option<Uuid>,
    pub is_trading: Option<bool>,
    pub supply: Option<i64>,
}

impl TokenPayload {
    pub fn new() -> Self {
        TokenPayload {
            user_id: None,
            id: None,
            ticker: None,
            is_trading: None,
            supply: None,

        }
    }
}

impl From<Token> for TokenPayload {
    fn from(token: Token) -> Self {
        TokenPayload {
            id: Some(token.id),
            ticker: Some(token.ticker), 
            user_id: Some(token.user_id),
            is_trading: Some(token.is_trading),
            supply: Some(token.supply),
       
        }
    }
}
// Associations,
// #[derive(Identifiable, Queryable, Serialize, Associations, Debug)]
// #[diesel(belongs_to(User, foreign_key = user_id))]
#[derive(Queryable, Selectable, Identifiable,  Debug, PartialEq)]
// #[diesel(belongs_to(User))]
#[diesel(table_name = tokens)]
pub struct Token {
    pub id: Uuid,
    pub ticker: String, 
    pub user_id: Uuid,
    pub is_trading: bool,
    pub supply: i64,
}


impl Token {
    pub async fn insert(
        mut payload: TokenPayload,
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


}
