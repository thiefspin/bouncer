use chrono::prelude::*;
use rocket_db_pools::Connection;

use crate::Users;
use crate::users::user_dao;
use crate::users::user_model::User;

pub async fn list_users(mut db: Connection<Users>) -> Vec<User> {
    return user_dao::list(db).await;
}

fn sast_date_time() -> DateTime<FixedOffset> {
    let utc: DateTime<Utc> = Utc::now();
    let offset = FixedOffset::east_opt(2 * 3600).unwrap();
    let sast: DateTime<FixedOffset> = DateTime::with_timezone(&utc, &offset);
    return sast;
}