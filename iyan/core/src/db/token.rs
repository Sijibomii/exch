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
// pub fn find_by_id(id: Uuid, conn: &mut PooledConnection) -> Result<Token, Error> {
//     use super::super::schema::tokens::dsl;

//     dsl::tokens
//         .filter(dsl::id.eq(id)) 
//         .first::<Token>(conn)
//         .map_err(|e| Error::from(e))
// }

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

// #[derive(Message)]
// #[rtype(result = "Result<Token, Error>")]
// pub struct FindById(pub Uuid);

// impl Handler<FindById> for PgExecutor {
//     type Result = Result<Token, Error>;

//     fn handle(&mut self, FindById(id): FindById, _: &mut Self::Context) -> Self::Result {
//         let conn = &mut self.get()?;

//         find_by_id(id, conn)
//     }
// }

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