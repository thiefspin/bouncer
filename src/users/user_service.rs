use rocket_db_pools::Connection;

use crate::Db;
use crate::users::user_dao;
use crate::users::user_model::User;

pub async fn list_users(db: Connection<Db>) -> Vec<User> {
    return user_dao::list(db).await;
}

pub async fn get(id: i64, db: Connection<Db>) -> Option<User> {
    return user_dao::get(id, db).await;
}

// fn sast_date_time() -> DateTime<FixedOffset> {
//     let utc: DateTime<Utc> = Utc::now();
//     let offset = FixedOffset::east_opt(2 * 3600).unwrap();
//     let sast: DateTime<FixedOffset> = DateTime::with_timezone(&utc, &offset);
//     return sast;
// }