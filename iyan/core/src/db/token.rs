
// insert
pub fn insert(payload: TokenPayload, conn: &mut PooledConnection) -> Result<User, Error> {
    use diesel::insert_into;
    use super::super::schema::users::dsl;

    insert_into(dsl::users)
        .values(&payload)
        .get_result(conn)
        .map_err(|e| Error::from(e))
}
// update 

// find by id

// delete