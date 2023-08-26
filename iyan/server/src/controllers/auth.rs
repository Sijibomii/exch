use actix_web::{web};
use actix_web::web::Json;
use super::super::state::AppState; 
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use super::super::services::{ self, errors::Error };
use core::user::{UserPayload};
use super::super::auth::AuthUser;
use uuid::Uuid;
#[derive(Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

// login 
pub async fn authentication(
    data: web::Json<LoginParams>,
    state: web::Data<AppState>
) -> Result<Json<Value>, Error>  {
    let params = data.into_inner();

    let res = services::users::authenticate(
        params.email,
        params.password,
        &state.postgres,
        state.jwt_private.clone(),
    ).await;

    match res {
        Ok((token, user)) => {
            return Ok(Json(json!({ "token": token, "user": user.export() })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    };
}

#[derive(Debug, Deserialize)]
pub struct RegistrationParams {
    pub email: String, 
    pub password: String,
}
// registration
pub async fn registration(
    data: web::Json<RegistrationParams>,
    state: web::Data<AppState>
) ->  Result<Json<Value>, Error> {
    
    let params = data.into_inner();

    if params.email.len() == 0 {
        return Err(Error::BadRequest("email is empty"));
    }

    if params.password.len() == 0 {
        return Err(Error::BadRequest("password is empty"));
    }

    let mut payload = UserPayload::new();
    payload.email = Some(params.email);
    payload.password = Some(params.password);

    let res = services::users::register(
        payload,
        &state.postgres
    ).await;

    match res {
        Ok(user) => {
            return Ok(Json(json!({ "user": user.export() })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    }
}

//  get the entire user
pub async fn profile( 
    state: web::Data<AppState>,
    user: AuthUser
) -> Result<Json<Value>, Error> {
    let res = services::users::get(user.id, &state.postgres).await;

    match res {
        Ok(user) => {
            return Ok(Json(json!({ "user": user.export() })))
        }
        Err(error) => {
            return Err(Error::from(error))
        }
    }
}

// this is how to get path parms in rust. the uuid is passed in
pub async fn delete(
    state: web::Data<AppState>, // Access the app state
    path: web::Path<Uuid>, // Extract dynamic path parameter
    user: AuthUser,
) -> Box<Result<Json<Value>, Error>> {
    let id = path.into_inner();
    if id != user.id {
        return Box::new(Err((Error::UnAuthorizedRequestAccount)));
    }
    let res = services::users::delete(user.id, &state.postgres).await;
    match res {
        Ok(us) => {
            return Box::new(Ok(Json(json!({ "deleted": us }))))
        }
        Err(error) => {
            return Box::new(Err(Error::from(error)))
        }
    }
}

