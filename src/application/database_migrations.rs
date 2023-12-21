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
            Err(err) => {
                println!("🔥 Migrations could not run successfully");
                println!("{}", err);
                Err(rocket)
            }
        }
    } else {
        Err(rocket)
    }
}