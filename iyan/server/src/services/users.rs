use chrono::{prelude::*, Duration};
// use std::future::Future;
use uuid::Uuid;
// use ring::{pbkdf2::{ PBKDF2_HMAC_SHA512 as PBKDF2_ALGORITHM}};
use std::num::NonZeroU32;
use ring::pbkdf2::{self, PBKDF2_HMAC_SHA512 as PBKDF2_ALGORITHM};
use super::super::auth::{AuthUser, JWTPayload};
use data_encoding::BASE64;
use core::{
    db::postgres::PgExecutorAddr,
    user::{User},
};
use super::errors::Error;


// const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
const N_ITER: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100000) };


pub async fn authenticate(
    email: String, 
    password: String,
    postgres: &PgExecutorAddr,
    jwt_private: Vec<u8>
) -> Result<(String, User), Error> {
    match User::find_by_email(email, postgres).await {
        Ok(user) => {
            // Do something with the user...
            let salt = BASE64
            .decode(&user.salt.as_bytes())
            .map_err(|e| Error::from(e))?;
            
            let password_hash = BASE64
            .decode(&user.password.as_bytes())
            // The .map_err(|e| Error::from(e)) method is called on the Result to map any potential base64::DecodeError into the desired Error type. 
            // It converts the base64::DecodeError into the Error type using the Error::from() conversion function or method.
            .map_err(|e| Error::from(e))?;
            
            pbkdf2::verify(
                PBKDF2_ALGORITHM,
                N_ITER,
                &salt,
                password.as_bytes(),
                &password_hash,
            )
            // If the verification fails, it maps the error to the Error::IncorrectPassword variant and converts it into a future.
            .map_err(|_| Error::IncorrectPassword).and_then(move |_| {
                let expires_at = Utc::now() + Duration::days(1);

                JWTPayload::new(Some(AuthUser { id: user.id }), None, expires_at)
                    .encode(&jwt_private)
                    .map_err(|e| Error::from(e))
                    // The .and_then(|token| Ok((token, user))) at the end of the chain converts the result into an Ok value, wrapping the tuple (token, user).
                    .and_then(|token| Ok((token, user)))
            })
        }
        Err(err) => {
            Err(err.into())
        }
    }

}