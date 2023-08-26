use actix::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;
use super::{Error, postgres::{PgExecutor, PooledConnection}};
use super::super::models::token::{Token, TokenPayload};


// insert
pub fn insert(payload: TokenPayload, conn: &mut PooledConnection) -> Result<Token, Error> {
    use diesel::insert_into;
    use super::super::schema::tokens::dsl;

    insert_into(dsl::tokens)
        .values(&payload)
        .get_result(conn)
        .map_err(|e| Error::from(e))
}

// update 
pub fn update(id: Uuid, payload: TokenPayload, conn: &mut PooledConnection) -> Result<Token, Error> {
    use diesel::update;
    use super::super::schema::tokens::dsl;
 
    update(dsl::tokens.filter(dsl::id.eq(id)))
        .set(&payload)
        .get_result(conn)
        .map_err(|e| Error::from(e))
}

// find by id
pub fn find_by_id(id: Uuid, conn: &mut PooledConnection) -> Result<Token, Error> {
    use super::super::schema::tokens::dsl;

    dsl::tokens
        .first::<Token>(conn)
        .map_err(|e| Error::from(e))
}

// delete
pub fn delete(id: Uuid, conn: &mut PooledConnection) -> Result<usize, Error> {
    use diesel::delete;
    use super::super::schema::tokens::dsl;

    delete(dsl::tokens.filter(dsl::id.eq(id)))
        .execute(conn)
        .map_err(|e| Error::from(e))?;

    Ok(1)
}