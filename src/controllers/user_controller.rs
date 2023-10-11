use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use rocket_okapi::openapi;

use crate::Users;
use crate::users::user_model::User;
use crate::users::user_service;

#[path = "../users/mod.rs"]
mod users;

#[openapi(tag = "Users")]
#[get("/")]
pub async fn list_users(mut db: Connection<Users>) -> Json<Vec<User>> {
    Json(user_service::list_users(db).await)
}

#[openapi(tag = "Users")]
#[get("/<id>")]
pub async fn get_user(mut db: Connection<Users>, id: i64) -> Option<Json<User>> {
    return user_service::list_users(db).await.iter().find(|u| u.id == id).map(|u| Json(u.to_owned()));
}