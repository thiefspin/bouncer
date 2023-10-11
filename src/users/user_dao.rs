use rocket_db_pools::Connection;
use rocket_db_pools::sqlx::*;

use crate::Users;
use crate::users::user_model::User;

pub async fn list(mut db: Connection<Users>) -> Vec<User> {
    let query_result = query_as!(
        User,
        "SELECT * FROM bouncer.users ORDER by id"
    ).fetch_all(&mut *db).await;
    return query_result.unwrap_or([].to_vec());
}