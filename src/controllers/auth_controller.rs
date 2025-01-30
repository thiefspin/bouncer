use rocket::{Route, State};
use rocket::http::Status;
use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::okapi::openapi3::OpenApi;
use crate::application::context::AppContext;
use crate::auth::auth_service;
use crate::auth::models::{AuthToken, LoginError, LoginForm, LoginResponse};
use crate::utils::controller_utils::BaseController;

pub struct AuthController;

impl BaseController for AuthController {
    fn routes() -> (Vec<Route>, OpenApi) {
        return openapi_get_routes_spec![
            login,
            validate
        ];
    }
}

#[openapi(tag = "Authentication")]
#[post("/login", format = "application/json", data = "<login_form>")]
pub async fn login(ctx: &State<AppContext>, login_form: Json<LoginForm>) -> Result<Json<LoginResponse>, Unauthorized<Json<LoginError>>> {
    match auth_service::login(ctx, login_form.into_inner()).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Unauthorized(Json(err)))
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