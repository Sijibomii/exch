use chrono::{prelude::*, Duration};
use futures::future::{err, Future, IntoFuture};
use uuid::Uuid;

use core::{
    db::postgres::PgExecutorAddr,
    user::{User, UserPayload},
};
use super::errors;


pub fn authenticate(
    email: String, 
    password: String,
    postgres: &PgExecutorAddr,
    jwt_private: Vec<u8>
)  -> impl Future<Item = (String, User), Error = Error> {

}