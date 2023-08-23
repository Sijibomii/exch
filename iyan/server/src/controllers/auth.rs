use actix_web::{web};
use actix_web::web::Json;
use crate::auth::AuthUser;
use futures::FutureExt;
use super::super::state::AppState; 
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use std::future::Future;
use super::super::services::{ self, errors::Error };
use core::user::{UserPayload, self};
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

// #[derive(Debug, Deserialize)]
// pub struct RegistrationParams {
//     pub email: String, 
//     pub password: String,
// }
// // registration
// pub fn registration(
//     data: web::Json<RegistrationParams>,
//     state: web::Data<AppState>
// ) ->  Result<Json<Value>, Error> {
//     let params = data.into_inner();

//     if params.email.len() == 0 {
//         return Box::new(Error::BadRequest("email is empty"));
//     }

//     if params.password.len() == 0 {
//         return Box::new(Error::BadRequest("password is empty"));
//     }

//     let mut payload = UserPayload::new();
//     payload.email = Some(params.email);
//     payload.password = Some(params.password);

//     let res = services::users::register(
//         payload,
//         state.mailer.clone(),
//         &state.postgres,
//         state.config.web_client_url.clone(),
//         state.config.mail_sender.clone(),
//     )
//     .then(|res| res.and_then(|user| Ok(Json(user.export()))));

//     HttpResponse::Ok().body(res);
// }


// #[derive(Deserialize)] 
// pub struct ActivationParams {
//     pub token: Uuid,
// }

// pub fn activation(
//     data: web::Json<ActivationParams>,
//     state: web::Data<AppState>
// ) ->  Result<Json<Value>, Error> {
//     let params = data.into_inner();
//     // activate. when email link is clicked
//     let res = services::users::activate(params.token, &state.postgres, state.jwt_private.clone()).then(
//         |res| {
//             res.and_then(|(token, user)| Ok(Json(json!({ "token": token, "user": user.export() }))))
//         },
//     );

//     HttpResponse::Ok().body(res);
// }

// #[derive(Deserialize)]
// pub struct ResetPasswordParams {
//     pub email: String,
// }

// pub fn reset_password(
//     data: web::Json<ResetPasswordParams>,
//     state: web::Data<AppState>
// ) -> Result<(), Error> {
//     let params = data.into_inner();

//     let res = services::users::reset_password( 
//         params.email,
//         state.mailer.clone(),
//         &state.postgres,
//         state.config.web_client_url.clone(),
//         state.config.mail_sender.clone(),
//     )
//     .then(|res| res.and_then(|_| Ok(Json(json!({})))));

//     HttpResponse::Ok().body(res);
// }

// #[derive(Deserialize)]
// pub struct ChangePasswordParams {
//     pub token: Uuid,
//     pub password: String,
// }

// pub fn change_password(
//     data: web::Json<ChangePasswordParams>,
//     state: web::Data<AppState>
// ) -> Result<Json<Value>, Error> {
//     let params = data.into_inner();

//     let res = services::users::change_password(
//         params.token,
//         params.password,
//         &state.postgres,
//         state.jwt_private.clone(),
//     )
//     .then(|res| {
//         res.and_then(|(token, user)| Ok(Json(json!({ "token": token, "user": user.export() }))))
//     });

//     HttpResponse::Ok().body(res);    
// }

// //  get the entire user
// pub fn profile( 
//     state: web::Data<AppState>,
//     user: AuthUser
// ) -> Result<Json<Value>, Error> {
//     let res = services::users::get(user.id, &state.postgres)
//         .then(|res| res.and_then(|user| Ok(Json(user.export()))));

//     HttpResponse::Ok().body(res);    
// }

// // this is how to get path parms in rust. the uuid is passed in
// pub fn delete(
//     // (state, path, user): (State<AppState>, Path<Uuid>, AuthUser),
// ) -> Box<Result<Json<Value>, Error>> {
//     let id = path.into_inner();
//     if id != user.id {
//         return Box::new(err(Error::InvalidRequestAccount));
//     }

//     Box::new(
//         services::users::delete(user.id, &state.postgres)
//             .then(|res| res.and_then(|deleted| Ok(Json(json!({ "deleted": deleted }))))),
//     )
// }