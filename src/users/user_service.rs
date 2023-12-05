use rocket_db_pools::Connection;

use crate::Db;
use crate::users::user_dao;
use crate::users::user_model::{User, UserCreateRequest};

pub async fn list_users(db: Connection<Db>) -> Vec<User> {
    return user_dao::list(db).await;
}

pub async fn get(id: i64, db: Connection<Db>) -> Option<User> {
    return user_dao::get(id, db).await;
}

pub async fn get_by_email(email: String, db: Connection<Db>) -> Option<User> {
    return user_dao::get_by_email(email, db).await;
}

pub async fn create(user: &UserCreateRequest, db: Connection<Db>) -> Option<User> {
    return user_dao::create(user, db).await;
}
