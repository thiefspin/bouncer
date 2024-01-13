use std::env;
use std::str::FromStr;
use std::time::Duration;

use log::LevelFilter;
use sqlx::{ConnectOptions, Pool, Postgres};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    pub port: u16,
    pub host: String,
    pub user: String,
    pub password: String,
    pub database_name: String,
    pub min_connections: u32,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn init() -> Self {
        let port = env::var("POSTGRES_PORT").expect("Please define an ENV var for POSTGRES_PORT").parse::<u16>().unwrap();
        let host = env::var("POSTGRES_HOST").expect("Please define an ENV var for POSTGRES_HOST");
        let user = env::var("POSTGRES_USER").expect("Please define an ENV var for POSTGRES_USER");
        let password = env::var("POSTGRES_PASSWORD").expect("Please define an ENV var for POSTGRES_PASSWORD");
        let database_name = env::var("POSTGRES_DATABASE_NAME").expect("Please define an ENV var for POSTGRES_DATABASE_NAME");
        let max_connections = match env::var("POSTGRES_MAX_CONNECTIONS") {
            Ok(conn) => conn.parse::<u32>().unwrap(),
            Err(_) => {
                info!("POSTGRES_MAX_CONNECTIONS not set. Defaulting to 4");
                4
            }
        };
        let min_connections = match env::var("POSTGRES_MIN_CONNECTIONS") {
            Ok(conn) => conn.parse::<u32>().unwrap(),
            Err(_) => {
                info!("POSTGRES_MIN_CONNECTIONS not set. Defaulting to 1");
                1
            }
        };
        DatabaseConfig {
            port,
            host,
            user,
            password,
            database_name,
            min_connections,
            max_connections,
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
        let options = PgConnectOptions::from_str(&database_url).unwrap()
            .disable_statement_logging()
            .log_slow_statements(LevelFilter::Warn, Duration::from_millis(500))
            .clone();
        let pool = match PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(2)
            .after_connect(|_conn, _meta| Box::pin(async move {
                debug!("Acquired new database connection");
                Ok(())
            }))
            .after_release(|_conn, meta| Box::pin(async move {
                debug!("Released database connection. Age: {} seconds, Idled: {} seconds", meta.age.as_secs(), meta.idle_for.as_secs());
                Ok(true)
            }))
            .connect_with(options)
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