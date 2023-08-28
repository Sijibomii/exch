use actix::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;
use super::{Error, postgres::{PgExecutor, PooledConnection}};
use super::super::models::token::{Token, TokenPayload};


pub fn insert(payload: TokenPayload, conn: &mut PooledConnection) -> Result<Token, Error> {
    use diesel::insert_into;
    use super::super::schema::tokens;
    // Build the insert statement
    let new_token = insert_into(tokens::table)
        .values(&payload)
        .returning(tokens::all_columns);

    // Execute the insert statement and get the inserted token
    let inserted_token: Token = new_token
        .get_result(conn)
        .map_err(Error::from)?;

    Ok(inserted_token)
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

// get all traded tokens
pub fn find_all_traded_tokens(
    limit: i64,
    offset: i64,
    conn: &mut PooledConnection,
) -> Result<Vec<Token>, Error> {
    use super::super::schema::tokens::dsl;

    dsl::tokens
        .filter(dsl::is_trading.eq(true))
        .limit(limit)
        .offset(offset)
        .load::<Token>(conn)
        .map_err(|e| Error::from(e))
}

// find by id
pub fn find_by_id(id: Uuid, conn: &mut PooledConnection) -> Result<Token, Error> {
    use super::super::schema::tokens::dsl;

    dsl::tokens
        .filter(dsl::id.eq(id)) 
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


#[derive(Message)]
#[rtype(result = "Result<Token, Error>")]
pub struct Insert(pub TokenPayload);

impl Handler<Insert> for PgExecutor {
    type Result = Result<Token, Error>;

    fn handle(&mut self, Insert(payload): Insert, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        insert(payload, conn)
    }
}


#[derive(Message)]
#[rtype(result = "Result<Token, Error>")]
pub struct Update {
    pub id: Uuid,
    pub payload: TokenPayload,
}

impl Handler<Update> for PgExecutor {
    type Result = Result<Token, Error>;

    fn handle(&mut self, Update { id, payload }: Update, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        update(id, payload, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Token>, Error>")]
pub struct FindAllTradedTokens {
    pub limit: i64,
    pub offset: i64,
} 

impl Handler<FindAllTradedTokens> for PgExecutor {
    type Result = Result<Vec<Token>, Error>;

    fn handle(
        &mut self,
        FindAllTradedTokens {
            limit,
            offset,
        }: FindAllTradedTokens,
        _: &mut Self::Context,
    ) -> Self::Result {
        let conn = &mut self.get()?;

        find_all_traded_tokens(limit, offset, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Token, Error>")]
pub struct FindById(pub Uuid);

impl Handler<FindById> for PgExecutor {
    type Result = Result<Token, Error>;

    fn handle(&mut self, FindById(id): FindById, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        find_by_id(id, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<usize, Error>")]
pub struct DeleteToken(pub Uuid);

impl Handler<DeleteToken> for PgExecutor {
    type Result = Result<usize, Error>;

    fn handle(&mut self, DeleteToken(id): DeleteToken, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        conn.transaction::<_, Error, _>(|conn| delete(id, conn))
    }
}