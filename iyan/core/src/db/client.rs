use actix::prelude::*;
use diesel::prelude::*;
use uuid::Uuid;
use super::{Error, postgres::{PgExecutor, PooledConnection}};
use super::super::models::client::{Client, ClientPayload};


pub fn insert(payload: ClientPayload, conn: &mut PooledConnection) -> Result<Client, Error> {
    use diesel::insert_into;
    use super::super::schema::client;
    // Build the insert statement
    let new_client = insert_into(client::table)
        .values(&payload)
        .returning(client::all_columns);

    // Execute the insert statement and get the inserted client
    let inserted_client: Client = new_client
        .get_result(conn)
        .map_err(Error::from)?;

    Ok(inserted_client)
}

// update 
pub fn update(id: Uuid, payload: ClientPayload, conn: &mut PooledConnection) -> Result<Client, Error> {
    use diesel::update;
    use super::super::schema::client::dsl;
 
    update(dsl::client.filter(dsl::id.eq(id)))
        .set(&payload)
        .get_result(conn)
        .map_err(|e| Error::from(e))
}

pub fn find_client(conn: &mut PooledConnection) -> Result<Client, Error> {
    use super::super::schema::client::dsl;

    dsl::client
        .load::<Client>(conn)
        .map_err(|e| Error::from(e))
        .and_then(|clients: Vec<Client>| {
            let ctl = clients
                .first()
                .cloned();

            if ctl.is_some() {
                return  Ok(ctl.unwrap());
            }else {
                return Ok(Client { id: Uuid::new_v4(), next_id: 0 })
            }
        })
}


#[derive(Message)]
#[rtype(result = "Result<Client, Error>")]
pub struct Insert(pub ClientPayload);

impl Handler<Insert> for PgExecutor {
    type Result = Result<Client, Error>;

    fn handle(&mut self, Insert(payload): Insert, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        insert(payload, conn)
    }
}

#[derive(Message)]
#[rtype(result = "Result<Client, Error>")]
pub struct Update {
    pub id: Uuid,
    pub payload: ClientPayload,
}

impl Handler<Update> for PgExecutor {
    type Result = Result<Client, Error>;

    fn handle(&mut self, Update { id, payload }: Update, _: &mut Self::Context) -> Self::Result {
        let conn = &mut self.get()?;

        update(id, payload, conn)
    }
}



#[derive(Message)]
#[rtype(result = "Result<Client, Error>")]
pub struct FindClient {} 

impl Handler<FindClient> for PgExecutor {
    type Result = Result<Client, Error>;

    fn handle(
        &mut self,
        FindClient {}: FindClient,
        _: &mut Self::Context,
    ) -> Self::Result {
        let conn = &mut self.get()?;
        find_client(conn)
    }
}