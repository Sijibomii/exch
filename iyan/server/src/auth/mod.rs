use actix_web::{error, Error as ActixError, FromRequest, web, HttpRequest, dev::Payload};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use futures::{future::{ready, Ready}, TryFutureExt};
use jsonwebtoken::{encode, EncodingKey, DecodingKey};
use uuid::Uuid;
use futures::future;


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

    pub fn encode(&self, jwt_private: &Vec<u8>) -> Result<String, jsonwebtoken::errors::Error> {
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256);
        let jwt_private = EncodingKey::from_rsa_pem(jwt_private).expect("Failed to create EncodingKey");
        encode(&header, &self, &jwt_private)
    }
}

impl FromRequest for JWTPayload {

    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>; 

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
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
            Ok(token) => ready(Ok(token.claims)),
            Err(_) => future::err(error::ErrorUnauthorized("invalid authorization token")),
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

        let token = JWTPayload::extract(&req);

       
        
      
    }
}

