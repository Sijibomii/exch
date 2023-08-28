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
pub async fn get_all_tokens(limit: i64, offset: i64,postgres: &PgExecutorAddr) -> Result<Vec<Token>, Error> {
    match Token::find_all_traded_tokens(limit, offset, postgres).await {
        Ok(token) => {
            Ok(token)
         }
         Err(err) => {
             Err(err.into())
         }
    }
}

// start trading 
pub async fn start_trading_token(id: Uuid, postgres: &PgExecutorAddr) -> Result<Token, Error> {
    // get token by id first 
    match Token::find_by_id(id, postgres).await {
        Ok(token) => {
            let mut payload = TokenPayload::from(token);
            payload.is_trading = Some(true);
            match Token::update(id, payload, postgres).await {
                Ok(ret_token) => {
                    Ok(ret_token)
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

pub async fn halt_trading_token(id: Uuid, postgres: &PgExecutorAddr) -> Result<Token, Error> {
    // get token by id first 
    match Token::find_by_id(id, postgres).await {
        Ok(token) => {
            let mut payload = TokenPayload::from(token);
            payload.is_trading = Some(false);
            match Token::update(id, payload, postgres).await {
                Ok(ret_token) => {
                    Ok(ret_token)
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

pub async fn delete(id: Uuid, postgres: &PgExecutorAddr) -> Result<usize, Error> {
    match Token::delete(id, postgres).await {
        Ok(u) => {
            Ok(u)
        }
        Err(err) => {
            Err(err.into())
        }
    }
} 