use rocket::futures::FutureExt;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rocket_okapi::openapi;

use crate::Db;
use crate::users::user_model::User;
use crate::users::user_service;

#[path = "../users/mod.rs"]
mod users;

#[openapi(tag = "Users")]
#[get("/")]
pub async fn list_users(db: Connection<Db>) -> Json<Vec<User>> {
    Json(user_service::list_users(db).await)
}

#[openapi(tag = "Users")]
#[get("/<id>")]
pub async fn get_user(db: Connection<Db>, id: i64) -> Option<Json<User>> {
    return user_service::get(id, db).map(|u_opt| u_opt.map(|u| Json(u))).await
}