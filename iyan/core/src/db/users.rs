use actix::prelude::*;
use chrono::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;

use super::{Error, postgres::{PgExecutor, PooledConnection}};
use super::super::models::user::{User, UserPayload}; 

// uses diesel to insert into the db 
pub fn insert(payload: UserPayload, conn: &mut PooledConnection) -> Result<User, Error> {
    use diesel::insert_into;
    use super::super::schema::users::dsl;

    insert_into(dsl::users)
        .values(&payload)
        
        .get_result(conn)
        .map_err(|e| Error::from(e))
        // .execute(conn)
}

pub fn update(id: Uuid, payload: UserPayload, conn: &mut PooledConnection) -> Result<User, Error> {
    use diesel::update;
    use super::super::schema::users::dsl;
 
    update(dsl::users.filter(dsl::id.eq(id)))
        .set(&payload)
        .get_result(conn)
        .map_err(|e| Error::from(e))
}

pub fn find_by_email(email: String, conn: &mut PooledConnection) -> Result<User, Error> {
    use super::super::schema::users::dsl;
    // @todo: add this back when the verification is put in place 
    // .and(dsl::is_verified.ne(false))
    dsl::users
        .filter(dsl::email.eq(email))
        .first::<User>(conn)
        .map_err(|e| Error::from(e))
}

pub fn find_by_id(id: Uuid, conn: &mut PooledConnection) -> Result<User, Error> {
    use super::super::schema::users::dsl;

    dsl::users
        .filter(dsl::id.eq(id))
        .first::<User>(conn)
        .map_err(|e| Error::from(e))
}

pub fn find_by_reset_token(token: Uuid, conn: &mut PooledConnection) -> Result<User, Error> {
    use super::super::schema::users::dsl;

    dsl::users
        .filter(
            dsl::reset_token
                .eq(token)
                .and(dsl::reset_token_expires_at.gt(Utc::now())),
        )
        .first::<User>(conn)
        .map_err(|e| Error::from(e))
}
// activate user account
pub fn activate(token: Uuid, conn: &mut PooledConnection) -> Result<User, Error> {
    use diesel::update;
    use super::super::schema::users::dsl;

    let mut payload = UserPayload::new();
    payload.is_verified = Some(true);

    // update(...) is a function provided by Diesel that constructs an update statement.
    update(
        // dsl::users.filter(...) creates a filter condition for the update, specifying the conditions under which rows should be updated.
        dsl::users.filter(
            // is_verified is false
            dsl::is_verified
                .ne(true)
                // vertification token equals token
                .and(dsl::verification_token.eq(token))
                // verification token has not expired.
                .and(dsl::verification_token_expires_at.gt(Utc::now())),
        ),
    )
    // .set(&payload) applies the provided payload object (which is assumed to be a struct or map) to update the selected rows with its field values.
    .set(&payload)
    // .get_result(conn) executes the update statement using the given database connection (conn). It returns the updated row as a result.
    .get_result(conn)
    // .map_err(|e| Error::from(e)) maps any error that occurs during the execution to a custom Error type, assuming the Error type implements the From trait for the error type returned by Diesel.
    .map_err(|e| Error::from(e))
}

pub fn delete(id: Uuid, conn: &mut PooledConnection) -> Result<usize, Error> {
    use diesel::delete;
    use super::super::schema::users::dsl;

    delete(dsl::users.filter(dsl::id.eq(id)))
        .execute(conn)
        .map_err(|e| Error::from(e))?;

    Ok(1)
}

pub fn delete_expired(email: String, conn: &mut PooledConnection) -> Result<usize, Error> {
    use diesel::delete;
    use super::super::schema::users::dsl;

    // delete(...) is a function provided by Diesel that constructs a delete statement.
    delete(
        // dsl::users.filter(...) creates a filter condition for the deletion, specifying the conditions under which rows should be deleted.
        dsl::users.filter(
            dsl::email
                // where email equals email, is_verified is false and verification_token_expires_at < now
                .eq(email)
                .and(dsl::is_verified.ne(true))
                .and(dsl::verification_token_expires_at.lt(Utc::now())),
        ),
    )
    // .execute(conn) executes the delete statement using the given database connection (conn). It returns the number of rows affected by the deletion.
    .execute(conn)
    // .map_err(|e| Error::from(e)) maps any error that occurs during the execution to a custom Error type, assuming the Error type implements the From trait for the error type returned by Diesel.
    .map_err(|e| Error::from(e))
}

// this recieves the insert message and inserts in the db
#[derive(Message)]
#[rtype(result = "Result<User, Error>")]
pub struct Insert(pub UserPayload);

impl Handler<Insert> for PgExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, Insert(payload): Insert, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        insert(payload, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<User, Error>")]
pub struct Update {
    pub id: Uuid,
    pub payload: UserPayload,
}

impl Handler<Update> for PgExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, Update { id, payload }: Update, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        update(id, payload, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<User, Error>")]
pub struct FindByEmail(pub String);

impl Handler<FindByEmail> for PgExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, FindByEmail(email): FindByEmail, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        find_by_email(email, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<User, Error>")]
pub struct FindById(pub Uuid);

impl Handler<FindById> for PgExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, FindById(id): FindById, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        find_by_id(id, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<User, Error>")]
pub struct FindByResetToken(pub Uuid);

impl Handler<FindByResetToken> for PgExecutor {
    type Result = Result<User, Error>;

    fn handle(
        &mut self,
        FindByResetToken(token): FindByResetToken,
        _: &mut Self::Context,
    ) -> Self::Result {
        let conn = &mut self.get()?;

        find_by_reset_token(token, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<User, Error>")]
pub struct Activate(pub Uuid);

impl Handler<Activate> for PgExecutor {
    type Result = Result<User, Error>;

    fn handle(&mut self, Activate(token): Activate, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        activate(token, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<usize, Error>")]
pub struct Delete(pub Uuid);

impl Handler<Delete> for PgExecutor {
    type Result = Result<usize, Error>;

    fn handle(&mut self, Delete(id): Delete, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        conn.transaction::<_, Error, _>(|conn| delete(id, conn))
    }
}

//  the code sets up a message handler for the DeleteExpired message, which is expected to handle the deletion of expired items using a PostgreSQL connection managed by the PgExecutor type.
#[derive(Message)]
#[rtype(result = "Result<usize, Error>")]
pub struct DeleteExpired(pub String);
//  han
impl Handler<DeleteExpired> for PgExecutor { 
    type Result = Result<usize, Error>;

    fn handle(
        &mut self,
        DeleteExpired(email): DeleteExpired,
        _: &mut Self::Context,
    ) -> Self::Result {
        let conn = &mut self.get()?;

        delete_expired(email, conn)
    }
}
