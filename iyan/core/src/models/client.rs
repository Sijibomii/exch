use uuid::Uuid;
use super::super::schema::{client};
use super::super::db::{
    postgres::PgExecutorAddr,
    client::{FindClient, Insert, Update}
};
use chrono::{prelude::*};
use super::super::models::errors::Error;
use diesel::prelude::*;
use serde_json::Value;

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = client)]
pub struct ClientPayload {
    pub id: Option<Uuid>,
    pub next_id: Option<i64>,
    pub next_ticker_id: Option<i64>,
}  

impl ClientPayload {
    pub fn new() -> Self {
        ClientPayload {
            id: None,
            next_id: None,
            next_ticker_id: None
        }
    }
}

impl From<Client> for ClientPayload {
    fn from(client: Client) -> Self {
        ClientPayload {
            id: Some(client.id),
            next_id: Some(client.next_id),
            next_ticker_id: Some(client.next_ticker_id)
        }
    } 
}

// Associations,
#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone)]
#[diesel(table_name = client)]
pub struct Client {
    pub id: Uuid,
    pub next_id: i64,
    pub next_ticker_id: i64,
}

impl Client {

    pub async fn insert(
        payload: ClientPayload,
        postgres: &PgExecutorAddr, 
    ) -> Result<Client, Error> {
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
        payload: ClientPayload,
        postgres: &PgExecutorAddr,
    ) -> Result<Client, Error> {

        (*postgres)
            .send(Update { id, payload })
            .await
            .map_err(Error::from)
            .and_then(|res| {
                res.map_err(|e| Error::from(e))
            })
    }

    // find client. client should have just one record so pick first
    pub async fn find_client(
        postgres: &PgExecutorAddr
    ) -> Result<Client, Error>  {
        (*postgres)
            .send(FindClient{})
            .await
            .map_err(Error::from)
            .and_then(|res| {
                res.map_err(|e| Error::from(e))
            })
    }
}