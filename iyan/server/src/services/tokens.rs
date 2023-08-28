use core::{db::postgres::{PgExecutorAddr}, token::{TokenPayload, Token}};
use super::errors::Error;

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


// get all tokens

