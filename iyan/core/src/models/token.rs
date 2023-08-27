use uuid::Uuid;
use super::super::schema::tokens;
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
    pub owner_id: Option<Uuid>,
    pub is_trading: Option<bool>,
    pub supply: Option<i64>
}

impl TokenPayload {
    pub fn new() -> Self {
        TokenPayload {
            owner_id: None,
            id: None,
            ticker: None,
            is_trading: None,
            supply: None
        }
    }
}

impl From<Token> for TokenPayload {
    fn from(token: Token) -> Self {
        TokenPayload {
            id: Some(token.id),
            ticker: Some(token.ticker), 
            owner_id: Some(token.owner_id),
            is_trading: Some(token.is_trading),
            supply: Some(token.supply)
        }
    }
}

#[derive(Debug, Identifiable, Queryable, Associations, Clone, Serialize, Deserialize)]
#[diesel(belongs_to(User, foreign_key = owner_id))]
pub struct Token {
    pub id: Uuid,
    pub ticker: String, 
    pub owner_id: Uuid,
    pub is_trading: bool,
    pub supply: i64
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
// -> impl Future<Item = Token, Error = Error> 