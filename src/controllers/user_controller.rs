use rocket::futures::FutureExt;
use rocket::response::status::Conflict;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rocket_okapi::openapi;
use crate::app::catchers::ApiError;
use crate::auth::auth_service::UserClaim;

use crate::Db;
use crate::users::user_model::{User, UserCreateRequest};
use crate::users::user_service;

#[path = "../users/mod.rs"]
mod users;

#[openapi(tag = "Users")]
#[get("/")]
pub async fn list_users(db: Connection<Db>, user: UserClaim) -> Json<Vec<User>> {
    println!("{} made an API call", user.user.name);
    Json(user_service::list_users(db).await)
}

#[openapi(tag = "Users")]
#[get("/<id>")]
pub async fn get_user(db: Connection<Db>, id: i64) -> Option<Json<User>> {
    return user_service::get(id, db).map(|u_opt| u_opt.map(|u| Json(u))).await
}

#[openapi(tag = "Users")]
#[post("/", format = "application/json", data = "<user>")]
pub async fn create(user: Json<UserCreateRequest>, db: Connection<Db>) -> Result<Json<User>, Conflict<Json<ApiError>>> {
    let inner = &user.into_inner();
    return match user_service::create(inner, db).await {
        Some(result) => Ok(Json(result)),
        None => Err(Conflict(Some(Json(ApiError{
            status: 409,
            message: format!("User with email {} already exists", inner.email)
        }))))
    }
}