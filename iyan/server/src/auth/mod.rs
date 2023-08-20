use actix_web::{error, Error as ActixError, FromRequest, HttpMessage, HttpRequest, dev::Payload};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use futures::future::{err, Future, ready, Ready};
use jsonwebtoken;
use uuid::Uuid;


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

        jsonwebtoken::encode(&header, &self, jwt_private)
    }
}

impl FromRequest for JWTPayload {

    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>; 

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let state = req.state();

        let auth_header = match req.headers().get("authorization") {
            Some(auth_header) => auth_header,
            None => return Err(error::ErrorUnauthorized("invalid authorization token")),
        };

        let auth_header_parts: Vec<_> = auth_header.to_str().unwrap().split_whitespace().collect();
        if auth_header_parts.len() != 2 {
            return Err(error::ErrorUnauthorized("invalid authorization token"));
        }

        if auth_header_parts.len() != 2 || auth_header_parts[0].to_lowercase() != "bearer" {
            return Err(error::ErrorUnauthorized("invalid authorization token"));
        }

        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256);

        match jsonwebtoken::decode::<JWTPayload>(&auth_header_parts[1], &state.jwt_public, &validation) {
            Ok(token) => Ok(token.claims),
            Err(_) => Err(error::ErrorUnauthorized("invalid authorization token")),
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
