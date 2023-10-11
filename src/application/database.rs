// use sqlx::{Pool, Postgres};
// use sqlx::postgres::PgPoolOptions;
//
// pub async fn init() -> Pool<Postgres> {
//     let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let _pool = match PgPoolOptions::new()
//         .max_connections(10)
//         .connect(&database_url)
//         .await
//     {
//         Ok(pool) => {
//             println!("✅Connection to the database is successful!");
//             return pool
//         }
//         Err(err) => {
//             println!("🔥 Failed to connect to the database: {:?}", err);
//             std::process::exit(1);
//         }
//     };
// }
//
// pub struct ApplicationDatabase{
//     pub pool: Pool<Postgres>
// }