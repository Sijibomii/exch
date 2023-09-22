use actix_web::{error, Error as ActixError, FromRequest, web, HttpRequest, dev::Payload};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use futures::{future::{ready, Ready}};
use jsonwebtoken::{encode, EncodingKey, DecodingKey};
use uuid::Uuid;
use futures::future;
use log::{debug};

use std::fs;

use super::state::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct JWTPayload {
    pub client: Option<AuthClient>,
    pub user: Option<AuthUser>,
    pub exp: u64,
}

impl JWTPayload {
    pub fn new(user: Option<AuthUser>, client: Option<AuthClient>, exp: DateTime<Utc>) -> Self {
        JWTPayload {
            client,
            user,
            exp: exp.timestamp() as u64,
        }
    }

    pub fn encode(&self, _: &Vec<u8>) -> Result<String, jsonwebtoken::errors::Error> {
        let private_key_data = fs::read("/keys/private_key.pem").expect("Failed to read private key file");
        debug!("jwt encode: successfully got into the encode func");
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
        debug!("jwt encode: successfully created header");
        let jwt_private = EncodingKey::from_rsa_pem(&private_key_data).expect("Failed to create EncodingKey");
        debug!("jwt encode: successfully created jwt private");
        
        let result = encode(&header, &self, &jwt_private);
        let res = result.clone();
        match result {
            Ok(_) => {
                debug!("jwt encode: got res successfully");
            }
            Err(e) => {
                debug!("jwt encode: errored!!!");
                // jwt encode error: "RSA key invalid: InvalidEncoding"
                debug!("jwt encode error: {:?}", e.to_string());
            }
        }
        return res;
    }
}

impl FromRequest for JWTPayload {

    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>; 

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let state = req.app_data::<web::Data<AppState>>().unwrap();
        
        let auth_header = match req.headers().get("authorization") {
            Some(auth_header) => {
                debug!("got auth header");
                auth_header
            }
            None => {
                debug!("failed to get auth header");
                return future::err(error::ErrorUnauthorized("invalid authorization token."))
            }
        };

        let auth_header_parts: Vec<_> = auth_header.to_str().unwrap().split_whitespace().collect();
        if auth_header_parts.len() != 2 {
            debug!("auth header error");
            return future::err(error::ErrorUnauthorized("invalid authorization token.."));
        }

        if auth_header_parts.len() != 2 || auth_header_parts[0].to_lowercase() != "bearer" {
            debug!("bearer error");
            return future::err(error::ErrorUnauthorized("invalid authorization token..."));
        }

        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        let jwt_public = DecodingKey::from_rsa_pem(&state.jwt_public).expect("Failed to create DecodingKey");
        match jsonwebtoken::decode::<JWTPayload>(&auth_header_parts[1], &jwt_public, &validation) {
            Ok(token) => ready(Ok(token.claims)),
            Err(_) => future::err(error::ErrorUnauthorized("invalid authorization token....")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthClient {
    pub id: Uuid,
    pub email: String,
    pub created_at: i64,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthUser {
    pub id: Uuid,
}

// implementation of the FromRequest trait from the Actix web framework for the AuthUser type.
impl FromRequest for AuthUser {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>; 

    // the from_request function is impl, this takes in a request and returns an AuthUser
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        // i should normally do this but I can't find a way to await the extract function without making from_request async.
        // the trait won't let me make it async.
        // let token = JWTPayload::extract(&req);
               
        let state = req.app_data::<web::Data<AppState>>().unwrap();

        let auth_header = match req.headers().get("authorization") {
            Some(auth_header) => auth_header,
            None => return future::err(error::ErrorUnauthorized("invalid authorization token")),
        };

        let auth_header_parts: Vec<_> = auth_header.to_str().unwrap().split_whitespace().collect();
        if auth_header_parts.len() != 2 {
            return future::err(error::ErrorUnauthorized("invalid authorization token"));
        }

        if auth_header_parts.len() != 2 || auth_header_parts[0].to_lowercase() != "bearer" {
            return future::err(error::ErrorUnauthorized("invalid authorization token"));
        }

        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);
        let jwt_public = DecodingKey::from_rsa_pem(&state.jwt_public).expect("Failed to create DecodingKey");
         match jsonwebtoken::decode::<JWTPayload>(&auth_header_parts[1], &jwt_public, &validation) {
            Ok(token) => {
                let tok = token.claims;
                match tok.user {
                    Some(user) => ready(Ok(user)),
                    None => future::err(error::ErrorUnauthorized("invalid authorization token")),
                }
            },
            Err(_) =>future::err(error::ErrorUnauthorized("invalid authorization token")),
        }
        
    }
}

