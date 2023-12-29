use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    pub port: u16,
    pub host: String,
    pub user: String,
    pub password: String,
    pub database_name: String
}

impl DatabaseConfig {
    pub fn init() -> Self {
        let port = env::var("POSTGRES_PORT").expect("Please define an ENV var for POSTGRES_PORT").parse::<u16>().unwrap();
        let host = env::var("POSTGRES_HOST").expect("Please define an ENV var for POSTGRES_HOST");
        let user = env::var("POSTGRES_USER").expect("Please define an ENV var for POSTGRES_USER");
        let password = env::var("POSTGRES_PASSWORD").expect("Please define an ENV var for POSTGRES_PASSWORD");
        let database_name = env::var("POSTGRES_DATABASE_NAME").expect("Please define an ENV var for POSTGRES_DATABASE_NAME");
        DatabaseConfig {
            port,
            host,
            user,
            password,
            database_name
        }
    }
}

#[derive(Clone, Debug)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn init(config: DatabaseConfig) -> Database {
        let database_url = format!("postgres://{}:{}@{}:{}/{}", config.user, config.password, config.host, config.port, config.database_name);
        let pool = match PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
        {
            Ok(pool) => {
                info!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                error!("🔥 Failed to connect to the database: {:?}", err);
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
                info!("✅ Migrations ran successfully");
            }
            Err(err) => {
                error!("🔥 Migrations could not run successfully. {}", err);
            }
        }
    }
}