use core::{db::postgres::{PgExecutorAddr}, token::{TokenPayload, Token}};
use super::errors::Error;
use uuid::Uuid;


//create a token
pub async fn create(
    payload: TokenPayload,
    postgres: &PgExecutorAddr,
) -> Result<Token, Error> {
    match Token::insert(payload, postgres).await {
        Ok(token) => {
            Ok(token)
        }

        Err(err) => {
            Err(err.into())
        }
    }
}

// get a token
pub async fn get(id: Uuid, postgres: &PgExecutorAddr) -> Result<Token, Error> {
    match Token::find_by_id(id, postgres).await {
        Ok(token) => {
            Ok(token)
         }
         Err(err) => {
             Err(err.into())
         }
    }
}

// get all tokens

