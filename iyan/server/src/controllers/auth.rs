use actix_web::{web, Responder, HttpResponse};
use super::super::state::AppState; // Import your AppState type
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

// login
pub fn authentication(
    (state, params): (State<AppState>, Json<LoginParams>),
) -> impl Future<Item = Json<Value>, Error = Error> {
    let params = params.into_inner();

    services::users::authenticate(
        params.email,
        params.password,
        &state.postgres,
        state.jwt_private.clone(),
    )
    .then(|res| {
        res.and_then(|(token, user)| Ok(Json(json!({ "token": token, "user": user.export() }))))
    })
}

#[derive(Debug, Deserialize)]
pub struct RegistrationParams {
    pub email: String, 
    pub password: String,
}
// registration
pub fn registration(
    (state, params): (State<AppState>, Json<RegistrationParams>),
) ->  impl Future<Item = Json<Value>, Error = Error> {
    let params = params.into_inner();

    if params.email.len() == 0 {
        return Box::new(err(Error::BadRequest("email is empty")));
    }

    if params.password.len() == 0 {
        return Box::new(err(Error::BadRequest("password is empty")));
    }

    let mut payload = UserPayload::new();
    payload.email = Some(params.email);
    payload.password = Some(params.password);

    services::users::register(
        payload,
        state.mailer.clone(),
        &state.postgres,
        state.config.web_client_url.clone(),
        state.config.mail_sender.clone(),
    )
    .then(|res| res.and_then(|user| Ok(Json(user.export()))))
}


#[derive(Deserialize)] 
pub struct ActivationParams {
    pub token: Uuid,
}

pub fn activation(
    (state, params): (State<AppState>, Json<ActivationParams>),
) -> impl Future<Item = Json<Value>, Error = Error> {
    let params = params.into_inner();
    // activate. when email link is clicked
    services::users::activate(params.token, &state.postgres, state.jwt_private.clone()).then(
        |res| {
            res.and_then(|(token, user)| Ok(Json(json!({ "token": token, "user": user.export() }))))
        },
    )
}

#[derive(Deserialize)]
pub struct ResetPasswordParams {
    pub email: String,
}

pub fn reset_password(
    (state, params): (State<AppState>, Json<ResetPasswordParams>),
) -> impl Future<Item = Json<Value>, Error = Error> {
    let params = params.into_inner();

    services::users::reset_password( 
        params.email,
        state.mailer.clone(),
        &state.postgres,
        state.config.web_client_url.clone(),
        state.config.mail_sender.clone(),
    )
    .then(|res| res.and_then(|_| Ok(Json(json!({})))))
}

#[derive(Deserialize)]
pub struct ChangePasswordParams {
    pub token: Uuid,
    pub password: String,
}

pub fn change_password(
    (state, params): (State<AppState>, Json<ChangePasswordParams>),
) -> impl Future<Item = Json<Value>, Error = Error> {
    let params = params.into_inner();

    services::users::change_password(
        params.token,
        params.password,
        &state.postgres,
        state.jwt_private.clone(),
    )
    .then(|res| {
        res.and_then(|(token, user)| Ok(Json(json!({ "token": token, "user": user.export() }))))
    })
}

//  get the entire user
pub fn profile( 
    (state, user): (State<AppState>, AuthUser),
) -> impl Future<Item = Json<Value>, Error = Error> {
    services::users::get(user.id, &state.postgres)
        .then(|res| res.and_then(|user| Ok(Json(user.export()))))
}

// this is how to get path parms in rust. the uuid is passed in
pub fn delete(
    (state, path, user): (State<AppState>, Path<Uuid>, AuthUser),
) -> Box<Future<Item = Json<Value>, Error = Error>> {
    let id = path.into_inner();
    if id != user.id {
        return Box::new(err(Error::InvalidRequestAccount));
    }

    Box::new(
        services::users::delete(user.id, &state.postgres)
            .then(|res| res.and_then(|deleted| Ok(Json(json!({ "deleted": deleted }))))),
    )
}