use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub struct DatabaseConfig {
    pub port: u16,
}

#[derive(Clone, Debug)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn init(config: DatabaseConfig) -> Database {
        let database_url = format!("postgres://service:password@localhost:{}/test", config.port);
        let pool = match PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
        {
            Ok(pool) => {
                println!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };
        Database { pool }
    }

    pub fn get_connection(&self) -> &Pool<Postgres> {
        &self.pool
    }

    pub async fn run_migrations(&self) {
        match sqlx::migrate!().run(&self.pool).await {
            Ok(_) => {
                println!("✅ Migrations ran successfully");
            }
            Err(err) => {
                println!("🔥 Migrations could not run successfully");
                println!("{}", err);
            }
        }
    }
}