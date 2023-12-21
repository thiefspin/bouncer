use rocket::State;

use crate::AppContext;
use crate::users::user_dao;
use crate::users::user_model::{User, UserCreateRequest};

pub async fn list_users(ctx: &State<AppContext>) -> Vec<User> {
    return user_dao::list(ctx.database.get_connection()).await;
}

pub async fn get(id: i64, ctx: &State<AppContext>) -> Option<User> {
    return user_dao::get(id, ctx.database.get_connection()).await;
}

pub async fn get_by_email(email: String, ctx: &State<AppContext>) -> Option<User> {
    return user_dao::get_by_email(email, ctx.database.get_connection()).await;
}

pub async fn create(user: &UserCreateRequest, ctx: &State<AppContext>) -> Option<User> {
    return user_dao::create(user, ctx.database.get_connection()).await;
}

pub async fn update_last_login(id: i64, ctx: &State<AppContext>) -> Option<User> {
    return user_dao::update_last_login(id, ctx.database.get_connection()).await;
}
