use rocket::http::Status;
use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;

use crate::{AppContext};
use crate::auth::auth_service;
use crate::auth::auth_token_validation::AuthToken;
use crate::auth::login_error::LoginError;
use crate::auth::login_form::LoginForm;
use crate::auth::login_response::LoginResponse;

#[openapi(tag = "Authentication")]
#[post("/login", format = "application/json", data = "<login_form>")]
pub async fn login(ctx: &State<AppContext>, login_form: Json<LoginForm>) -> Result<Json<LoginResponse>, Unauthorized<Json<LoginError>>> {
    match auth_service::login(ctx, login_form.into_inner()).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Unauthorized(Some(Json(err))))
    }
}

#[openapi(tag = "Authentication")]
#[post("/validate", format = "application/json", data = "<token>")]
pub async fn validate(token: Json<AuthToken>) -> Status {
    if auth_service::validate(token.into_inner()).await {
        Status::NoContent
    } else {
        Status::Unauthorized
    }
}