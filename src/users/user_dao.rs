use rocket_db_pools::sqlx::*;

use crate::users::user_model::{User, UserCreateRequest};
use crate::utils::date_time::DateUtils;

pub async fn list(conn: &PgPool) -> Vec<User> {
    let query_result = query_as!(
        User,
        "SELECT * FROM bouncer.users ORDER by id"
    ).fetch_all(conn).await;
    return query_result.unwrap();
}

pub async fn get(id: i64, db: &PgPool) -> Option<User> {
    let query_result = query_as!(
        User,
        "SELECT * FROM bouncer.users WHERE id = $1", id
    ).fetch_optional(db).await;
    return query_result.unwrap();
}

pub async fn get_by_email(email: String, conn: &PgPool) -> Option<User> {
    let query_result = query_as!(
        User,
        "SELECT * FROM bouncer.users WHERE email = $1", email
    ).fetch_optional(conn).await;
    return query_result.unwrap();
}

pub async fn create(user: &UserCreateRequest, conn: &PgPool) -> Option<User> {
    let query_result = query_as!(
        User,
        "INSERT INTO bouncer.users (email, name, surname, phone, password, created) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        user.email, user.name, user.surname, user.phone, user.password, DateUtils::sast_now().naive_utc()
    ).fetch_optional(conn).await;
    return match query_result {
        Ok(result) => result,
        Err(_) => None
    }
}

pub async fn update_last_login(id: i64, conn: &PgPool) -> Option<User> {
    let query_result = query_as!(
        User,
        "UPDATE bouncer.users SET last_login = $1 WHERE id = $2 RETURNING *",
        DateUtils::sast_now().naive_utc(), id
    ).fetch_optional(conn).await;
    return match query_result {
        Ok(result) => result,
        Err(_) => None
    }
}