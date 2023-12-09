use rocket::{Build, fairing, Rocket};
use rocket_db_pools::Database;
use crate::Db;

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    if let Some(db) = Db::fetch(&rocket) {
        match sqlx::migrate!().run(&db.0).await {
            Ok(_) => {
                println!("✅ Migrations ran successfully");
                Ok(rocket)
            }
            Err(_) => {
                println!("🔥 Migrations could not run successfully");
                Err(rocket)
            }
        }
    } else {
        Err(rocket)
    }
}