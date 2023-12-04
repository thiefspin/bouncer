use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rocket_okapi::openapi;

use crate::auth::auth_service;
use crate::auth::login_error::LoginError;
use crate::auth::login_form::LoginForm;
use crate::auth::login_response::LoginResponse;
use crate::Db;

#[openapi(tag = "Authentication")]
#[post("/login", format = "application/json", data = "<login_form>")]
pub async fn login(db: Connection<Db>, login_form: Json<LoginForm>) -> Result<Json<LoginResponse>, Unauthorized<Json<LoginError>>> {
    match auth_service::login(db, login_form.into_inner()).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(Unauthorized(Some(Json(err))))
    }
}