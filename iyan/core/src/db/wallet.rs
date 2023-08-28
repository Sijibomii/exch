use actix::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;
use super::{Error, postgres::{PgExecutor, PooledConnection}};
use super::super::models::wallet::{Wallet, WalletPayload};

pub fn insert(payload: WalletPayload, conn: &mut PooledConnection) -> Result<Wallet, Error> {
    use diesel::insert_into;
    use super::super::schema::wallet;
    // Build the insert statement
    let new_wallet = insert_into(wallet::table)
        .values(&payload)
        .returning(wallet::all_columns);

    // Execute the insert statement and get the inserted wallet
    let inserted_wallet: Wallet = new_wallet
        .get_result(conn)
        .map_err(Error::from)?;

    Ok(inserted_wallet)
}

// update 
pub fn update(id: Uuid, payload: WalletPayload, conn: &mut PooledConnection) -> Result<Wallet, Error> {
    use diesel::update;
    use super::super::schema::wallet::dsl;
 
    update(dsl::wallet.filter(dsl::id.eq(id)))
        .set(&payload)
        .get_result(conn)
        .map_err(|e| Error::from(e))
}

pub fn find_all_wallets_by_user(id: Uuid, limit: i64, offset: i64, conn: &mut PooledConnection) -> Result<Vec<Wallet>, Error> {
    use super::super::schema::wallet::dsl;

    dsl::wallet
        .filter(dsl::user_id.eq(id))
        .limit(limit)
        .offset(offset)
        .load::<Wallet>(conn)
        .map_err(|e| Error::from(e))
}

// find by id
pub fn find_by_id(id: Uuid, conn: &mut PooledConnection) -> Result<Wallet, Error> {
    use super::super::schema::wallet::dsl;

    dsl::wallet
        .filter(dsl::id.eq(id)) 
        .first::<Wallet>(conn)
        .map_err(|e| Error::from(e))
}

// delete
pub fn delete(id: Uuid, conn: &mut PooledConnection) -> Result<usize, Error> {
    use diesel::delete;
    use super::super::schema::wallet::dsl;

    delete(dsl::wallet.filter(dsl::id.eq(id)))
        .execute(conn)
        .map_err(|e| Error::from(e))?;

    Ok(1)
}



#[derive(Message)]
#[rtype(result = "Result<Wallet, Error>")]
pub struct Insert(pub WalletPayload);

impl Handler<Insert> for PgExecutor {
    type Result = Result<Wallet, Error>;

    fn handle(&mut self, Insert(payload): Insert, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        insert(payload, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Wallet, Error>")]
pub struct Update {
    pub id: Uuid,
    pub payload: WalletPayload,
}

impl Handler<Update> for PgExecutor {
    type Result = Result<Wallet, Error>;

    fn handle(&mut self, Update { id, payload }: Update, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        update(id, payload, conn)
    }
}



#[derive(Message)]
#[rtype(result = "Result<Vec<Wallet>, Error>")]
pub struct FindAllWalletsByUser {
    pub limit: i64,
    pub offset: i64,
    pub id: Uuid
} 

impl Handler<FindAllWalletsByUser> for PgExecutor {
    type Result = Result<Vec<Wallet>, Error>;

    fn handle(
        &mut self,
        FindAllWalletsByUser {
            id,
            limit,
            offset,
        }: FindAllWalletsByUser,
        _: &mut Self::Context,
    ) -> Self::Result {
        let conn = &mut self.get()?;
        find_all_wallets_by_user(id, limit, offset, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Wallet, Error>")]
pub struct FindWalletById(pub Uuid);

impl Handler<FindWalletById> for PgExecutor {
    type Result = Result<Wallet, Error>;

    fn handle(&mut self, FindWalletById(id): FindWalletById, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        find_by_id(id, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<usize, Error>")]
pub struct DeleteWallet(pub Uuid);

impl Handler<DeleteWallet> for PgExecutor {
    type Result = Result<usize, Error>;

    fn handle(&mut self, DeleteWallet(id): DeleteWallet, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        conn.transaction::<_, Error, _>(|conn| delete(id, conn))
    }
}