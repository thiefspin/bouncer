use rocket::fairing;
use sqlx::PgPool;

// pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
//     if let Some(db) = Db::fetch(&rocket) {
//         match sqlx::migrate!().run(&db.0).await {
//             Ok(_) => {
//                 println!("✅ Migrations ran successfully");
//                 Ok(rocket)
//             }
//             Err(err) => {
//                 println!("🔥 Migrations could not run successfully");
//                 println!("{}", err);
//                 Err(rocket)
//             }
//         }
//     } else {
//         Err(rocket)
//     }
// }

// pub async fn run_migrations(db: PgPool) -> fairing::Result {
//     match sqlx::migrate!().run(pool).await {
//         Ok(_) => {
//             println!("✅ Migrations ran successfully");
//             Ok(rocket)
//         }
//         Err(err) => {
//             println!("🔥 Migrations could not run successfully");
//             println!("{}", err);
//             Err(rocket)
//         }
//     }
// }