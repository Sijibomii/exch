
use chrono::{prelude::*, Duration};
// use std::future::Future;
use std::num::NonZeroU32;
use ring::{pbkdf2::{self, PBKDF2_HMAC_SHA512 as PBKDF2_ALGORITHM}, rand, digest, rand::SecureRandom};
use super::super::auth::{AuthUser, JWTPayload};
use data_encoding::BASE64;
use core::{
    db::postgres::PgExecutorAddr,
    user::{User, UserPayload},
};
use uuid::Uuid;
use super::errors::Error;
use log::{debug};

const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
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
            debug!("service: got user successfully");
            let salt = BASE64
            .decode(&user.salt.as_bytes())
            .map_err(|e| Error::from(e))?;
            debug!("service: decoded salt successfully");
            let password_hash = BASE64
            .decode(&user.password.as_bytes())
            // The .map_err(|e| Error::from(e)) method is called on the Result to map any potential base64::DecodeError into the desired Error type. 
            // It converts the base64::DecodeError into the Error type using the Error::from() conversion function or method.
            .map_err(|e| Error::from(e))?;
            debug!("service: decoded password hash successfully");
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
                debug!("service: verified successfully");
                debug!("service: jwt payload: {:?}", jwt_private);
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

pub async fn register(
    mut payload: UserPayload,
    postgres: &PgExecutorAddr
) -> Result<User, Error> {

    let postgres = postgres.clone();
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; CREDENTIAL_LEN];
    // rng.fill(&mut salt).unwrap(); fills the salt array with random bytes generated by the rng random number generator. 
    rng.fill(&mut salt).unwrap();

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

    pbkdf2::derive(
        // SHA512 specifies the hash function to use for PBKDF2, in this case, SHA-512.
        PBKDF2_ALGORITHM,
        // N_ITER represents the number of iterations for the PBKDF2 algorithm.
        N_ITER,
        // &salt is the generated salt used in the key derivation.
        &salt,
        // payload.password.unwrap().as_bytes() retrieves the password from the payload (presumably containing the user's input) and converts it to bytes.
        payload.password.unwrap().as_bytes(),
        // &mut pbkdf2_hash is the mutable reference to the array where the derived PBKDF2 hash will be stored.
        &mut pbkdf2_hash,
    );

    // This is commonly done to ensure that the encoded hash and salt can be easily transmitted or stored as strings,
    //  which is convenient for various use cases, such as storing in a database or including them in JSON payloads.
    // BASE64.encode(&pbkdf2_hash) encodes the derived PBKDF2 hash (pbkdf2_hash) as a Base64-encoded string. 
    payload.password = Some(BASE64.encode(&pbkdf2_hash));
    // BASE64.encode(&salt) encodes the generated salt (salt) as a Base64-encoded string.
    payload.salt = Some(BASE64.encode(&salt));

    // insert user
    match User::insert(payload, &postgres).await {
        Ok(user) => {
           Ok(user)
        }
        Err(err) => {
            Err(err.into())
        }
    }
} 

// get a user by id 
pub async fn get(id: Uuid, postgres: &PgExecutorAddr) -> Result<User, Error> {
    match User::find_by_id(id, postgres).await {
        Ok(user) => {
            Ok(user)
         }
         Err(err) => {
             Err(err.into())
         }
    }
}

pub async fn delete(id: Uuid, postgres: &PgExecutorAddr) -> Result<usize, Error>{
    match User::delete(id, postgres).await {
        Ok(_) => {
            Ok(0)
        }
        Err(err) => {
            Err(err.into())
        }
    }
}
