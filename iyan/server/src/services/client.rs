use core::{db::postgres::{PgExecutorAddr}, client::{ Client, ClientPayload}};
use super::errors::Error;


// insert client count
pub async fn insert(
    payload: ClientPayload,
    postgres: &PgExecutorAddr,
) -> Result<Client, Error> {
    match Client::insert(payload, postgres).await {
        Ok(client) => {
            Ok(client)
        }

        Err(err) => {
            Err(err.into())
        }
    }
}

// get client count
pub async fn get_client_count(postgres: &PgExecutorAddr) -> Result<Client, Error>  {
    match Client::find_client(postgres).await {
        Ok(client) => {
            Ok(client)
        }
        Err(err) => {
            Err(err.into())
        }
    }
}


// increase client count
pub async fn increase_client_count(
    postgres: &PgExecutorAddr,
) -> Result<Client, Error> {
    
    match Client::find_client(postgres).await {
        Ok(client) => {
            let mut payload = ClientPayload::from(client);
            payload.next_id =if payload.next_id.is_some() { Some(payload.next_id.unwrap() + 1) } else { Some(1) };

            match Client::update(payload.id.unwrap(), payload, postgres).await {
                Ok(ret_client) => {
                    Ok(ret_client)
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

