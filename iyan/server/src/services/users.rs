

pub fn authenticate(
    email: String, 
    password: String,
    postgres: &PgExecutorAddr,
    jwt_private: Vec<u8>
)  -> impl Future<Item = (String, User), Error = Error> {

}