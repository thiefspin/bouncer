use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::*;

use crate::Db;
use crate::users::user_model::User;

pub async fn list(mut db: Connection<Db>) -> Vec<User> {
    let query_result = query_as!(
        User,
        "SELECT * FROM bouncer.users ORDER by id"
    ).fetch_all(&mut *db).await;
    return query_result.unwrap();
}

pub async fn get(id: i64, mut db: Connection<Db>) -> Option<User> {
    let query_result = query_as!(
        User,
        "SELECT * FROM bouncer.users WHERE id = $1", id
    ).fetch_optional(&mut *db).await;
    return query_result.unwrap();
}

pub async fn get_by_email(email: String, mut db: Connection<Db>) -> Option<User> {
    let query_result = query_as!(
        User,
        "SELECT * FROM bouncer.users WHERE email = $1", email
    ).fetch_optional(&mut *db).await;
    return query_result.unwrap();
}