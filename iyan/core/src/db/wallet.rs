use actix::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;
use super::{Error, postgres::{PgExecutor, PooledConnection}};
use super::super::models::wallet::{Wallet, WalletPayload};

pub fn insert(payload: WalletPayload, conn: &mut PooledConnection) -> Result<Wallet, Error> {
    use diesel::insert_into;
    use super::super::schema::wallet;
    // Build the insert statement
    let new_token = insert_into(wallet::table)
        .values(&payload)
        .returning(wallet::all_columns);

    // Execute the insert statement and get the inserted token
    let inserted_token: Wallet = new_token
        .get_result(conn)
        .map_err(Error::from)?;

    Ok(inserted_token)
}
